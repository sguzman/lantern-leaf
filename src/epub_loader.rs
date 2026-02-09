//! EPUB loading utilities.
//!
//! This module is intentionally small: it knows how to open an EPUB, walk
//! through its spine, strip basic markup, and return a single `String` of text.
//! Keeping it isolated makes it easy to swap out or enhance parsing later
//! (e.g., extracting a table of contents or preserving styling).

use anyhow::{Context, Result};
use epub::doc::EpubDoc;
use std::fs;
use std::path::Path;
use tracing::{debug, info, warn};

/// Load an EPUB from disk and return its text content as a single string.
pub fn load_epub_text(path: &Path) -> Result<String> {
    if is_plain_text(path) {
        info!(path = %path.display(), "Loading plain text content");
        let data = fs::read_to_string(path)
            .with_context(|| format!("Failed to read text file at {}", path.display()))?;
        let text = if data.trim().is_empty() {
            "No textual content found in this file.".to_string()
        } else {
            data
        };
        info!(
            total_chars = text.len(),
            "Finished loading plain text content"
        );
        return Ok(text);
    }

    info!(path = %path.display(), "Loading EPUB content");
    let mut doc =
        EpubDoc::new(path).with_context(|| format!("Failed to open EPUB at {}", path.display()))?;

    let mut combined = String::new();
    let mut chapters = 0usize;

    loop {
        match doc.get_current_str() {
            Some((chapter, _mime)) => {
                chapters += 1;
                if !combined.is_empty() {
                    combined.push_str("\n\n");
                }
                // Use a lightweight HTML-to-text pass to remove most markup; fall back to raw chapter on errors.
                // Use a very large width so we do not bake in hard line breaks—let the UI handle wrapping.
                let plain = match html2text::from_read(chapter.as_bytes(), 10_000) {
                    Ok(clean) => clean,
                    Err(err) => {
                        warn!(chapter = chapters, "html2text failed: {err}");
                        chapter
                    }
                };
                debug!(
                    chapter = chapters,
                    added_chars = plain.len(),
                    "Parsed chapter"
                );
                combined.push_str(&plain);
            }
            None => break,
        }

        if !doc.go_next() {
            break;
        }
    }

    if combined.trim().is_empty() {
        combined.push_str("No textual content found in this EPUB.");
    }

    info!(
        chapters,
        total_chars = combined.len(),
        "Finished loading EPUB content"
    );
    Ok(combined)
}

fn is_plain_text(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase()),
        Some(ext) if ext == "txt" || ext == "md" || ext == "markdown"
    )
}
