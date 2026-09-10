use super::*;

impl ReaderSession {
    pub fn load(
        source_path: PathBuf,
        config: config::AppConfig,
        normalizer: &normalizer::TextNormalizer,
        bookmark: Option<crate::cache::Bookmark>,
    ) -> Result<Self, String> {
        Self::load_with_cancel(source_path, config, normalizer, bookmark, None)
    }

    pub fn load_with_cancel(
        source_path: PathBuf,
        mut config: config::AppConfig,
        normalizer: &normalizer::TextNormalizer,
        bookmark: Option<crate::cache::Bookmark>,
        cancel: Option<&CancellationToken>,
    ) -> Result<Self, String> {
        let loaded = epub_loader::load_book_content_with_cancel(&source_path, cancel)
            .map_err(|err| format!("{err:#}"))?;
        let source_name = crate::cache::load_browser_tab_manifest(&source_path)
            .map(|manifest| manifest.title.trim().to_string())
            .filter(|title| !title.is_empty())
            .or_else(|| {
                source_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "book".to_string());

        config.font_size = config
            .font_size
            .clamp(pagination::MIN_FONT_SIZE, pagination::MAX_FONT_SIZE);
        config.lines_per_page = config.lines_per_page.clamp(
            pagination::MIN_LINES_PER_PAGE,
            pagination::MAX_LINES_PER_PAGE,
        );

        if !config.dual_view_pipeline_enabled {
            tracing::warn!(
                path = %source_path.display(),
                "Config field dual_view_pipeline_enabled=false is deprecated; dual view pipeline is now always enabled"
            );
        }
        if matches!(
            config.native_html_pagination_mode,
            config::NativeHtmlPaginationMode::ChapterSection
        ) {
            tracing::info!(
                path = %source_path.display(),
                mode = "chapter_section",
                "Native HTML pagination mode configured; sentence indexing continuity remains canonical and drives page transitions"
            );
        }
        let reading_markdown = loaded.reading_markdown;
        let reading_html = loaded.reading_html;
        let structured_document = loaded.structured_document.map(Arc::new);
        let has_structured_markdown = loaded.has_structured_markdown;
        let cached_pdf_sync = crate::cache::load_pdf_sync_meta(&source_path);
        let pdf_geometry_mode = loaded.pdf_geometry_mode.or_else(|| {
            cached_pdf_sync
                .as_ref()
                .map(|value| value.pdf_geometry_mode)
        });
        let pdf_sync_strategy = loaded.pdf_sync_strategy.or_else(|| {
            cached_pdf_sync
                .as_ref()
                .map(|value| value.pdf_sync_strategy)
        });
        let pdf_classification = loaded.pdf_classification.or_else(|| {
            cached_pdf_sync
                .as_ref()
                .and_then(|value| value.pdf_classification.clone())
        });
        let pdf_runtime_policy = loaded.pdf_runtime_policy.or_else(|| {
            cached_pdf_sync
                .as_ref()
                .and_then(|value| value.pdf_runtime_policy.clone())
        });

        let mut session = Self {
            source_path,
            source_name,
            tts_text: loaded.tts_text,
            reading_markdown,
            reading_html,
            structured_document,
            has_structured_markdown,
            pdf_geometry_mode,
            pdf_sync_strategy,
            pdf_classification,
            pdf_runtime_policy,
            pdf_ocr_alignment: None,
            pdf_ocr_pipeline: loaded.pdf_ocr_pipeline,
            images: loaded
                .images
                .into_iter()
                .map(|image| {
                    let path = fs::canonicalize(&image.path).unwrap_or(image.path);
                    SessionImage {
                        raw_path: image.source_ref,
                        path: path.to_string_lossy().to_string(),
                    }
                })
                .collect(),
            config,
            book_overrides: config::BookReaderOverrides::default(),
            pages: Vec::new(),
            markdown_pages: Vec::new(),
            raw_page_sentences: Vec::new(),
            sentence_anchor_maps: Vec::new(),
            page_word_counts: Vec::new(),
            page_sentence_counts: Vec::new(),
            current_page: 0,
            highlighted_display_idx: None,
            highlighted_canonical_idx: None,
            highlighted_audio_idx: None,
            text_only_mode: false,
            search_query: String::new(),
            search_matches: Vec::new(),
            selected_search_match: None,
            tts_state: TtsPlaybackState::Idle,
            current_plan_page: None,
            current_plan_display_start: 0,
            current_plan_display_end: 0,
            current_plan: None,
            snapshot_constructions: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        };

        if let Some(bookmark) = bookmark.as_ref() {
            session.current_page = bookmark.page;
        }
        let repagination_started = std::time::Instant::now();
        session.repaginate(normalizer, None);
        tracing::info!(
            path = %session.source_path.display(),
            pages = session.pages.len(),
            sentences = session.page_sentence_counts.iter().sum::<usize>(),
            elapsed_ms = repagination_started.elapsed().as_millis() as u64,
            "Finished bounded reader repagination and sentence-anchor setup"
        );
        if let Some(bookmark) = bookmark.as_ref() {
            session.restore_bookmark_position(bookmark, normalizer);
        }
        if session.highlighted_display_idx.is_none() {
            session.highlighted_display_idx = Some(0).filter(|_| session.current_display_len() > 0);
        }
        Ok(session)
    }
}

pub fn load_session_for_source(
    source_path: PathBuf,
    base_config: &config::AppConfig,
    normalizer: &normalizer::TextNormalizer,
) -> Result<ReaderSession, String> {
    load_session_for_source_with_cancel(source_path, base_config, normalizer, None)
}

pub fn load_session_for_source_with_cancel(
    source_path: PathBuf,
    base_config: &config::AppConfig,
    normalizer: &normalizer::TextNormalizer,
    cancel: Option<&CancellationToken>,
) -> Result<ReaderSession, String> {
    let mut effective_config = base_config.clone();
    let book_overrides = crate::cache::load_book_reader_overrides(&source_path).unwrap_or_default();
    book_overrides.apply_to(&mut effective_config);
    let bookmark = crate::cache::load_bookmark(&source_path);
    let mut session = ReaderSession::load_with_cancel(
        source_path,
        effective_config,
        normalizer,
        bookmark,
        cancel,
    )?;
    session.book_overrides = book_overrides;
    Ok(session)
}

pub fn persist_session_housekeeping_with_cache(
    session: &ReaderSession,
    cache_service: &dyn crate::cache_service::CacheService,
) {
    let bookmark = session.to_bookmark();
    cache_service.save_bookmark(Path::new(&session.source_path), &bookmark);
    cache_service
        .save_book_reader_overrides(Path::new(&session.source_path), &session.book_overrides);
}

pub fn persist_session_housekeeping(session: &ReaderSession) {
    let cache_service = crate::cache_service::FilesystemCacheService;
    persist_session_housekeeping_with_cache(session, &cache_service);
}
