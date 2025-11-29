//! Pagination utilities.
//!
//! The strategy here is intentionally simple: we approximate how many
//! characters fit on a page based on the chosen font size, then split the
//! text into fixed-size chunks. The logic is isolated so it can be swapped for
//! a more sophisticated layout later.

/// Minimum allowed font size (points).
pub const MIN_FONT_SIZE: u32 = 12;
/// Maximum allowed font size (points).
pub const MAX_FONT_SIZE: u32 = 36;

/// Split the provided text into page-sized chunks based on the font size.
pub fn paginate(text: &str, font_size: u32) -> Vec<String> {
    let normalized = font_size.clamp(MIN_FONT_SIZE, MAX_FONT_SIZE) as f32;

    // Roughly scale page size as font size changes. These constants are easy to
    // tweak while keeping the function deterministic.
    let chars_per_line = (80.0 * (16.0 / normalized))
        .round()
        .clamp(30.0, 120.0) as usize;
    let lines_per_page = (28.0 * (16.0 / normalized))
        .round()
        .clamp(10.0, 80.0) as usize;
    let chars_per_page = chars_per_line.saturating_mul(lines_per_page).max(1);

    // Split into paragraphs, preserving order. We consider a blank line as a
    // paragraph boundary, which matches how `html2text` emits content.
    let paragraphs = split_paragraphs(text);
    if paragraphs.is_empty() {
        return vec![String::new()];
    }

    let mut pages = Vec::new();
    let mut current = String::new();
    let mut current_len = 0usize;

    for para in paragraphs {
        // Paragraph length plus a separating blank line if not first on page.
        let separator_len = if current.is_empty() { 0 } else { 2 }; // "\n\n"
        let para_len = para.len();
        let prospective_len = current_len + separator_len + para_len;

        if !current.is_empty() && prospective_len > chars_per_page {
            // Finish the current page and start a new one.
            pages.push(std::mem::take(&mut current));
            current_len = 0;
        }

        if current.is_empty() {
            // Start the page with this paragraph (may exceed page size; we
            // still keep paragraphs intact).
            current.push_str(&para);
            current_len = para_len;
        } else {
            current.push_str("\n\n");
            current.push_str(&para);
            current_len += separator_len + para_len;
        }
    }

    if !current.is_empty() {
        pages.push(current);
    }

    pages
}

/// Split text into paragraphs separated by blank lines.
fn split_paragraphs(text: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut buffer = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            if !buffer.is_empty() {
                paragraphs.push(buffer.join("\n"));
                buffer.clear();
            }
        } else {
            buffer.push(line);
        }
    }

    if !buffer.is_empty() {
        paragraphs.push(buffer.join("\n"));
    }

    paragraphs
}
