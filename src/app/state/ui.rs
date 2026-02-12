use crate::cache::RecentBook;
use crate::calibre::{CalibreBook, CalibreColumn, CalibreConfig};

pub struct SearchState {
    pub(in crate::app) visible: bool,
    pub(in crate::app) query: String,
    pub(in crate::app) error: Option<String>,
    pub(in crate::app) matches: Vec<usize>,
    pub(in crate::app) selected_match: usize,
}

pub struct RecentState {
    pub(in crate::app) visible: bool,
    pub(in crate::app) books: Vec<RecentBook>,
}

pub struct CalibreState {
    pub(in crate::app) visible: bool,
    pub(in crate::app) loading: bool,
    pub(in crate::app) error: Option<String>,
    pub(in crate::app) books: Vec<CalibreBook>,
    pub(in crate::app) search_query: String,
    pub(in crate::app) config: CalibreConfig,
    pub(in crate::app) sort_column: CalibreColumn,
    pub(in crate::app) sort_desc: bool,
}
