//! Pagination utilities.
//!
//! The strategy here is intentionally simple: we split text into fixed-size
//! chunks based on a stable character budget so page count remains steady
//! even when font size changes. The logic is isolated so it can be swapped
//! for a more sophisticated layout later.
use crate::text_utils::split_sentences;

/// Minimum allowed font size (points).
pub const MIN_FONT_SIZE: u32 = 12;
/// Maximum allowed font size (points).
pub const MAX_FONT_SIZE: u32 = 36;
/// Minimum lines per page.
pub const MIN_LINES_PER_PAGE: usize = 8;
/// Maximum lines per page.
pub const MAX_LINES_PER_PAGE: usize = 1000;

/// Split the provided text into page-sized chunks.
pub fn paginate(text: &str, font_size: u32, lines_per_page: usize) -> Vec<String> {
    let _ = font_size.clamp(MIN_FONT_SIZE, MAX_FONT_SIZE); // kept for signature compatibility
    let lines = lines_per_page.clamp(MIN_LINES_PER_PAGE, MAX_LINES_PER_PAGE);

    // Keep a stable page size regardless of font size so page count does not
    // jump when the user tweaks text size. Font size still affects wrapping at
    // render time, but pagination is based on a fixed character budget.
    const CHARS_PER_LINE: usize = 80;
    let chars_per_page = CHARS_PER_LINE.saturating_mul(lines).max(1);
    let sentences = split_sentences(text);
    if sentences.is_empty() {
        return vec![String::new()];
    }

    let mut pages = Vec::new();
    let mut current_sentences: Vec<String> = Vec::new();
    let mut current_len = 0usize;

    for sentence in sentences {
        let sentence = sentence.trim();
        if sentence.is_empty() {
            continue;
        }
        let sentence_len = sentence.chars().count();
        let separator_len = if current_sentences.is_empty() { 0 } else { 1 }; // " "
        let prospective_len = current_len + separator_len + sentence_len;

        if !current_sentences.is_empty() && prospective_len > chars_per_page {
            pages.push(current_sentences.join(" "));
            current_sentences.clear();
            current_len = 0;
        }

        if !current_sentences.is_empty() {
            current_len += 1;
        }
        current_sentences.push(sentence.to_string());
        current_len += sentence_len;
    }

    if !current_sentences.is_empty() {
        pages.push(current_sentences.join(" "));
    }

    if pages.is_empty() {
        vec![String::new()]
    } else {
        pages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_utils::split_sentences;

    #[test]
    fn pagination_preserves_sentence_text_across_page_sizes() {
        let sentence = "This sentence is intentionally long so that we can force pagination without splitting sentence content. ";
        let mut text = String::new();
        for i in 0..40 {
            text.push_str(&format!("{i}: {sentence}"));
            text.push('.');
            text.push(' ');
        }

        let canonical: Vec<String> = split_sentences(&text)
            .into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        for lines in [8usize, 12, 40, 120] {
            let pages = paginate(&text, 16, lines);
            let rebuilt: Vec<String> = pages
                .into_iter()
                .flat_map(|p| split_sentences(&p))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            assert_eq!(
                rebuilt, canonical,
                "sentence corpus changed at lines_per_page={lines}"
            );
        }
    }
}
