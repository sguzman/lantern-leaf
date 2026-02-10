//! Text splitting helpers for TTS alignment.

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::PathBuf;

/// Very lightweight sentence splitter based on punctuation.
pub fn split_sentences(text: &str) -> Vec<String> {
    split_sentences_with_abbreviations(text, &ABBREVIATION_TOKENS)
}

fn split_sentences_with_abbreviations(text: &str, abbreviations: &HashSet<String>) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = text.chars().collect();

    for (idx, ch) in chars.iter().copied().enumerate() {
        current.push(ch);
        if matches!(ch, '.' | '!' | '?') && !period_is_abbreviation(&chars, idx, abbreviations) {
            if current.chars().any(|c| !c.is_whitespace()) {
                sentences.push(current.clone());
            }
            current.clear();
        }
    }

    if current.chars().any(|c| !c.is_whitespace()) {
        sentences.push(current);
    }

    sentences
}

fn period_is_abbreviation(chars: &[char], dot_idx: usize, abbreviations: &HashSet<String>) -> bool {
    if chars.get(dot_idx).copied() != Some('.') {
        return false;
    }

    let mut start = dot_idx;
    while start > 0 && chars[start - 1].is_alphabetic() {
        start -= 1;
    }
    if start == dot_idx {
        return false;
    }

    let token: String = chars[start..dot_idx].iter().collect();
    if token.is_empty() {
        return false;
    }

    let lookup = format!("{}.", token.to_ascii_lowercase());
    if abbreviations.contains(&lookup) {
        return true;
    }

    if token.len() == 1 {
        // Treat interior periods in initialisms like "U.S." as non-terminal.
        if start >= 2 && chars[start - 1] == '.' && chars[start - 2].is_alphabetic() {
            return true;
        }

        // Also avoid splitting at the first period when another "X." follows.
        let mut next = dot_idx + 1;
        while next < chars.len() && chars[next].is_whitespace() {
            next += 1;
        }
        if next + 1 < chars.len() && chars[next].is_alphabetic() && chars[next + 1] == '.' {
            return true;
        }
    }

    false
}

static ABBREVIATION_TOKENS: Lazy<HashSet<String>> = Lazy::new(load_abbreviation_tokens);

fn load_abbreviation_tokens() -> HashSet<String> {
    let mut out = HashSet::new();
    for default in ["mr.", "ms.", "mrs.", "mass.", "st."] {
        out.insert(default.to_string());
    }

    let path = PathBuf::from("conf/normalizer.toml");
    let Ok(contents) = fs::read_to_string(&path) else {
        return out;
    };
    if let Ok(file) = toml::from_str::<NormalizerFile>(&contents) {
        for key in file.normalization.abbreviations.keys() {
            let normalized = normalize_abbreviation_token(key);
            if !normalized.is_empty() {
                out.insert(normalized);
            }
        }
    }
    out
}

fn normalize_abbreviation_token(raw: &str) -> String {
    let trimmed = raw.trim().trim_end_matches('.');
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("{}.", trimmed.to_ascii_lowercase())
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct NormalizerFile {
    normalization: NormalizationConfig,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct NormalizationConfig {
    abbreviations: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::split_sentences;

    #[test]
    fn does_not_split_common_abbreviations() {
        let text = "Mr. Smith walked in. Mrs. Jones stayed.";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 2);
    }

    #[test]
    fn keeps_initialism_together() {
        let text = "This uses U.S. spelling. Next sentence.";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 2);
    }
}
