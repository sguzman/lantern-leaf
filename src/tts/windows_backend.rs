use anyhow::{Context, Result, anyhow};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use windows::Media::SpeechSynthesis::SpeechSynthesizer;
use windows::Storage::Streams::DataReader;
use windows::core::HSTRING;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct WindowsVoiceDescriptor {
    pub id: String,
    pub display_name: String,
    pub language: String,
    pub gender: String,
}

pub fn resolve_voice_id(configured_id: Option<&str>) -> Result<String> {
    resolve_voice_id_with_preference(configured_id, "Zira")
}

pub fn resolve_voice_id_with_preference(
    configured_id: Option<&str>,
    preferred_name: &str,
) -> Result<String> {
    let voices = SpeechSynthesizer::AllVoices()?;
    if let Some(configured_id) = configured_id {
        for index in 0..voices.Size()? {
            let voice = voices.GetAt(index)?;
            if voice.Id()?.to_string_lossy() == configured_id {
                return Ok(configured_id.to_string());
            }
        }
        return Err(anyhow!("Windows voice ID was not found: {configured_id}"));
    }
    let normalized_preference = normalize_voice_name(preferred_name);
    if !normalized_preference.is_empty() {
        let mut candidates = Vec::new();
        for index in 0..voices.Size()? {
            let voice = voices.GetAt(index)?;
            let display_name = voice.DisplayName()?.to_string_lossy();
            let normalized_name = normalize_voice_name(&display_name);
            if normalized_name == normalized_preference
                || normalized_name
                    .split_whitespace()
                    .any(|token| token == normalized_preference)
            {
                candidates.push((
                    voice.Language()?.to_string_lossy().to_ascii_lowercase(),
                    voice.Id()?.to_string_lossy(),
                ));
            }
        }
        candidates.sort_by(|left, right| {
            let left_en = !left.0.starts_with("en-us");
            let right_en = !right.0.starts_with("en-us");
            (left_en, &left.1).cmp(&(right_en, &right.1))
        });
        if let Some((_, id)) = candidates.into_iter().next() {
            tracing::info!(preferred_name, voice_id = %id, "Resolved Windows voice preference");
            return Ok(id);
        }
        tracing::warn!(
            preferred_name,
            "Preferred Windows voice unavailable; using OS default"
        );
    }
    Ok(SpeechSynthesizer::DefaultVoice()?.Id()?.to_string_lossy())
}

fn normalize_voice_name(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn enumerate_voices() -> Result<Vec<WindowsVoiceDescriptor>> {
    SpeechSynthesizer::new().context("initializing Windows speech synthesizer")?;
    let voices = SpeechSynthesizer::AllVoices().context("enumerating Windows voices")?;
    let mut result = Vec::with_capacity(voices.Size()? as usize);
    for index in 0..voices.Size()? {
        let voice = voices.GetAt(index)?;
        result.push(WindowsVoiceDescriptor {
            id: voice.Id()?.to_string_lossy(),
            display_name: voice.DisplayName()?.to_string_lossy(),
            language: voice.Language()?.to_string_lossy(),
            gender: format!("{:?}", voice.Gender()),
        });
    }
    result.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(result)
}

pub fn synthesize_sentence_to_wav(text: &str, path: &Path, voice_id: Option<&str>) -> Result<()> {
    let synthesizer = SpeechSynthesizer::new().context("creating Windows speech synthesizer")?;
    if let Some(id) = voice_id {
        let voices = SpeechSynthesizer::AllVoices()?;
        let mut selected = None;
        for index in 0..voices.Size()? {
            let candidate = voices.GetAt(index)?;
            if candidate.Id()?.to_string_lossy() == id {
                selected = Some(candidate);
                break;
            }
        }
        let voice = selected.ok_or_else(|| anyhow!("Windows voice ID was not found: {id}"))?;
        synthesizer.SetVoice(&voice)?;
    }
    let stream = synthesizer
        .SynthesizeTextToStreamAsync(&HSTRING::from(text))?
        .join()
        .context("waiting for Windows speech synthesis")?;
    let content_type = stream.ContentType()?.to_string_lossy();
    if !content_type.eq_ignore_ascii_case("audio/wav") {
        return Err(anyhow!(
            "Windows speech synthesis returned unsupported content type: {content_type}"
        ));
    }
    let size = stream.Size()?;
    if size == 0 || size > u32::MAX as u64 {
        return Err(anyhow!(
            "Windows speech synthesis returned invalid stream size: {size}"
        ));
    }
    let input = stream.GetInputStreamAt(0)?;
    let reader = DataReader::CreateDataReader(&input)?;
    reader
        .LoadAsync(size as u32)?
        .join()
        .context("reading Windows speech stream")?;
    let mut bytes = vec![0u8; size as usize];
    reader.ReadBytes(&mut bytes)?;
    let temp_path = path.with_extension("wav.partial");
    let mut file =
        File::create(&temp_path).with_context(|| format!("creating {}", temp_path.display()))?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    drop(file);
    std::fs::rename(&temp_path, path).with_context(|| format!("publishing {}", path.display()))?;
    Ok(())
}
