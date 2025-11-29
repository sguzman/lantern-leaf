//! Entry point for the EPUB viewer.
//!
//! Responsibilities here are intentionally minimal:
//! - Parse command-line arguments.
//! - Load the EPUB text via `epub_loader`.
//! - Launch the GUI application with the loaded text.

mod app;
mod epub_loader;
mod pagination;

use crate::app::run_app;
use crate::epub_loader::load_epub_text;
use anyhow::{anyhow, Context, Result};
use std::env;
use std::path::PathBuf;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let epub_path = parse_args()?;
    let text = load_epub_text(&epub_path)?;
    run_app(text).context("Failed to start the GUI")?;
    Ok(())
}

fn parse_args() -> Result<PathBuf> {
    let mut args = env::args().skip(1);
    let path = args
        .next()
        .ok_or_else(|| anyhow!("Usage: epub-viewer <path-to-epub>"))?;

    let path = PathBuf::from(path);
    if !path.exists() {
        return Err(anyhow!(
            "File not found: {}",
            path.as_path().display()
        ));
    }
    Ok(path)
}
