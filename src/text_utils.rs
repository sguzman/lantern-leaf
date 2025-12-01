//! Text splitting helpers for TTS alignment.

/// Very lightweight sentence splitter based on punctuation.
pub fn split_sentences(text: String) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        current.push(ch);
        if matches!(ch, '.' | '!' | '?') {
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
