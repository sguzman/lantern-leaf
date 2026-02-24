#![allow(dead_code)]

pub mod chunk_plan;
pub mod config;
pub mod engine;
pub mod pipeline;
pub mod policy;
pub mod postprocess;
pub mod probe;
pub mod report;
pub mod util;

use crate::cancellation::CancellationToken;
use crate::quack_check::config::Config;
use crate::quack_check::engine::python::PythonEngine;
use crate::quack_check::pipeline::Pipeline;
use crate::quack_check::util::{ensure_dir, hash_file, sha256_hex};
use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use tracing::warn;

#[derive(Debug, Clone)]
pub struct RunResult {
    pub job_id: String,
    pub job_dir: PathBuf,
    pub text: String,
}

pub fn run_pdf_to_text(config_path: &Path, input: &Path, out_root: &Path) -> Result<RunResult> {
    run_pdf_to_text_with_cancel(config_path, input, out_root, None)
}

pub fn run_pdf_to_text_with_cancel(
    config_path: &Path,
    input: &Path,
    out_root: &Path,
    cancel: Option<CancellationToken>,
) -> Result<RunResult> {
    let cfg = Config::load(config_path)?;
    validate_input(&cfg, input)?;

    let cfg_hash = sha256_hex(cfg.normalized_for_hash().as_bytes());
    let input_hash = hash_file(&cfg, input)?;
    let job_id = sha256_hex(format!("{cfg_hash}:{input_hash}").as_bytes());
    let job_dir = out_root.join(&job_id);

    if job_dir.exists() && !cfg.global.resume {
        return Err(anyhow!(
            "quack-check job_dir already exists and resume=false: {}",
            job_dir.display()
        ));
    }

    ensure_dir(&job_dir)?;
    ensure_dir(&job_dir.join("final"))?;
    ensure_dir(&job_dir.join("chunks"))?;
    if cfg.logging.write_to_file {
        ensure_dir(&job_dir.join("logs"))?;
    }

    if cfg.debug.dump_effective_config {
        let raw = toml::to_string(&cfg).unwrap_or_default();
        std::fs::write(job_dir.join("effective-config.toml"), raw)?;
    }

    if !cfg.paths.work_dir.trim().is_empty() {
        ensure_dir(Path::new(&cfg.paths.work_dir))?;
    }
    if !cfg.paths.cache_dir.trim().is_empty() {
        ensure_dir(Path::new(&cfg.paths.cache_dir))?;
    }
    if !cfg.paths.docling_artifacts_dir.trim().is_empty() {
        ensure_dir(Path::new(&cfg.paths.docling_artifacts_dir))?;
    }

    let engine = PythonEngine::new_with_cancel(&cfg, cancel.clone())?;
    let pipeline = Pipeline::new_with_cancel(&cfg, engine, cancel);
    let result = pipeline.run_job(input, &job_dir)?;

    if cfg.output.write_markdown {
        std::fs::write(
            job_dir.join("final").join(&cfg.output.markdown_filename),
            &result.markdown,
        )?;
    }
    if cfg.output.write_text {
        std::fs::write(
            job_dir.join("final").join(&cfg.output.text_filename),
            &result.text,
        )?;
    }
    if cfg.output.write_report_json {
        std::fs::write(
            job_dir.join("final").join(&cfg.output.report_filename),
            serde_json::to_string_pretty(&result.report)?,
        )?;
    }
    if cfg.output.write_index_json {
        let index = serde_json::json!({
            "job_id": job_id,
            "job_dir": job_dir,
            "status": "ok",
            "final_markdown": format!("final/{}", cfg.output.markdown_filename),
            "final_text": format!("final/{}", cfg.output.text_filename),
            "report": format!("final/{}", cfg.output.report_filename),
        });
        std::fs::write(
            job_dir.join("index.json"),
            serde_json::to_string_pretty(&index)?,
        )?;
    }

    Ok(RunResult {
        job_id,
        job_dir,
        text: result.text,
    })
}

fn validate_input(cfg: &Config, input: &Path) -> Result<()> {
    let input_str = input.display().to_string();

    if cfg.security.reject_url_inputs && looks_like_url(&input_str) {
        return Err(anyhow!("URL inputs are disabled: {input_str}"));
    }

    if !input.exists() {
        return Err(anyhow!("input does not exist: {}", input.display()));
    }

    if let Some(ext) = input.extension().and_then(|s| s.to_str()) {
        if ext.to_ascii_lowercase() != "pdf" {
            return Err(anyhow!("input is not a PDF: {}", input.display()));
        }
    } else {
        warn!("input has no extension; assuming PDF: {}", input.display());
    }

    Ok(())
}

fn looks_like_url(s: &str) -> bool {
    let s = s.to_ascii_lowercase();
    s.starts_with("http://") || s.starts_with("https://") || s.starts_with("file://")
}
