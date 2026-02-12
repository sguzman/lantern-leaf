use crate::epub_loader::BookImage;

/// Reader-related model.
pub struct ReaderState {
    pub(in crate::app) full_text: String,
    pub(in crate::app) pages: Vec<String>,
    pub(in crate::app) page_sentences: Vec<Vec<String>>,
    pub(in crate::app) page_sentence_counts: Vec<usize>,
    pub(in crate::app) images: Vec<BookImage>,
    pub(in crate::app) current_page: usize,
}

impl ReaderState {
    pub(in crate::app) fn set_page_clamped(&mut self, page: usize) {
        if self.pages.is_empty() {
            self.current_page = 0;
        } else {
            self.current_page = page.min(self.pages.len().saturating_sub(1));
        }
    }
}
