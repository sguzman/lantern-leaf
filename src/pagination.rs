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
    let mut chars_per_page = chars_per_line.saturating_mul(lines_per_page).max(1);

    // Guard against overflow for extremely small or extreme values.
    if chars_per_page == 0 {
        chars_per_page = 1;
    }

    let mut pages = Vec::new();
    let mut current = String::new();
    let mut count = 0usize;

    for ch in text.chars() {
        current.push(ch);
        count += 1;

        if count >= chars_per_page {
            pages.push(std::mem::take(&mut current));
            count = 0;
        }
    }

    if !current.is_empty() {
        pages.push(current);
    }

    pages
}
