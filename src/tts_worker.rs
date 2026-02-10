use hound::WavSpec;
use piper_rs::from_config_path;
use piper_rs::synth::PiperSpeechSynthesizer;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct WorkerRequest {
    text: Option<String>,
    path: Option<String>,
    shutdown: Option<bool>,
}

#[derive(Serialize)]
struct WorkerResponse {
    ok: bool,
    error: Option<String>,
}

pub fn maybe_run_worker() -> bool {
    if env::args().any(|arg| arg == "--tts-worker") {
        if let Err(err) = run_worker() {
            eprintln!("tts-worker error: {err}");
        }
        return true;
    }
    false
}

fn run_worker() -> anyhow::Result<()> {
    let mut args = env::args().skip_while(|arg| arg != "--tts-worker");
    let _ = args.next();

    let mut model_path: Option<PathBuf> = None;
    let mut espeak_root: Option<PathBuf> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--model" => {
                if let Some(path) = args.next() {
                    model_path = Some(PathBuf::from(path));
                }
            }
            "--espeak" => {
                if let Some(path) = args.next() {
                    espeak_root = Some(PathBuf::from(path));
                }
            }
            _ => {}
        }
    }

    let model_path = model_path.ok_or_else(|| anyhow::anyhow!("Missing --model argument"))?;
    let espeak_root = espeak_root.ok_or_else(|| anyhow::anyhow!("Missing --espeak argument"))?;

    if env::var_os("PIPER_ESPEAKNG_DATA_DIRECTORY").is_none() {
        // Safe because the worker runs in a dedicated process before threads are spawned.
        unsafe {
            env::set_var("PIPER_ESPEAKNG_DATA_DIRECTORY", &espeak_root);
        }
    }

    let config_path = resolve_piper_config(&model_path);
    if !config_path.exists() {
        anyhow::bail!(
            "Piper config not found at {} (expected from {})",
            config_path.display(),
            model_path.display()
        );
    }
    let model = from_config_path(&config_path)?;
    let piper = PiperSpeechSynthesizer::new(model)?;

    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    let mut stdout = std::io::stdout();

    loop {
        line.clear();
        let read = reader.read_line(&mut line)?;
        if read == 0 {
            break;
        }
        let req: WorkerRequest = serde_json::from_str(line.trim())
            .map_err(|err| anyhow::anyhow!("Invalid request: {err}"))?;

        if req.shutdown.unwrap_or(false) {
            break;
        }

        let result = match (req.text, req.path) {
            (Some(text), Some(path)) => {
                let path = PathBuf::from(path);
                synthesize_to_file_serial(&piper, &path, &text)
            }
            _ => Err(anyhow::anyhow!("Invalid request payload")),
        };

        let response = match result {
            Ok(()) => WorkerResponse {
                ok: true,
                error: None,
            },
            Err(err) => WorkerResponse {
                ok: false,
                error: Some(err.to_string()),
            },
        };
        let payload = serde_json::to_string(&response)?;
        stdout.write_all(payload.as_bytes())?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
    }

    Ok(())
}

fn resolve_piper_config(model_path: &Path) -> PathBuf {
    if model_path
        .extension()
        .map(|ext| ext == "onnx")
        .unwrap_or(false)
    {
        return model_path.with_extension("onnx.json");
    }
    model_path.to_path_buf()
}

fn synthesize_to_file_serial(
    piper: &PiperSpeechSynthesizer,
    path: &Path,
    sentence: &str,
) -> anyhow::Result<()> {
    let output_config = None;
    let mut samples: Vec<f32> = Vec::new();
    let mut sample_rate: Option<u32> = None;
    let mut channels: Option<u16> = None;
    for chunk in piper.synthesize_lazy(sentence.to_string(), output_config)? {
        let chunk = chunk?;
        if sample_rate.is_none() {
            sample_rate = Some(chunk.info.sample_rate as u32);
            channels = Some(chunk.info.num_channels as u16);
        }
        samples.extend_from_slice(chunk.samples.as_slice());
    }

    if samples.is_empty() {
        anyhow::bail!("No speech data to write");
    }

    write_wav(
        path,
        sample_rate.unwrap_or(22050),
        channels.unwrap_or(1),
        &samples,
    )?;

    Ok(())
}

fn write_wav(path: &Path, sample_rate: u32, channels: u16, samples: &[f32]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let temp_path = unique_temp_wav_path(path);
    let mut writer = hound::WavWriter::create(&temp_path, spec)?;
    for &s in samples {
        let clamped = (s * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        writer.write_sample(clamped)?;
    }
    writer.finalize()?;
    if fs::rename(&temp_path, path).is_err() {
        fs::copy(&temp_path, path)?;
        let _ = fs::remove_file(&temp_path);
    }
    Ok(())
}

fn unique_temp_wav_path(path: &Path) -> PathBuf {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let nonce = SEQ.fetch_add(1, Ordering::Relaxed);
    let ts_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut temp_name = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("tts.wav")
        .to_string();
    temp_name.push_str(&format!(".tmp-{}-{nonce}", ts_nanos));
    path.with_file_name(temp_name)
}
