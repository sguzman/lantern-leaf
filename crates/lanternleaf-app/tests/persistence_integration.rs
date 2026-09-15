use lanternleaf_app::persistence::{
    FilesystemPersistenceService, PersistenceLifecycle, ReaderHousekeeping,
};
use lanternleaf_app::pipeline::PersistenceTrigger;
use lanternleaf_core::{cache, config, session};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

static CACHE_LOCK: Mutex<()> = Mutex::new(());

fn unique_source_path(ext: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("lanternleaf_persist_{nanos}.{ext}"))
}

fn unique_cache_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("lanternleaf_cache_{nanos}"))
}

fn with_cache_root<F: FnOnce()>(test_body: F) {
    let _guard = CACHE_LOCK.lock().expect("cache lock");
    let cache_root = unique_cache_root();
    unsafe { std::env::set_var(cache::CACHE_DIR_ENV, &cache_root) };
    let _ = fs::create_dir_all(&cache_root);
    test_body();
    unsafe { std::env::remove_var(cache::CACHE_DIR_ENV) };
    let _ = fs::remove_dir_all(cache_root);
}
fn write_source(path: &Path) {
    fs::write(path, "test content").expect("write source");
}

fn write_epub_source(path: &Path) {
    let paragraphs = (0..180)
        .map(|index| {
            format!(
                "<p>Persistent EPUB paragraph {index} remains complete after close and reopen.</p>"
            )
        })
        .collect::<String>();
    let entries = vec![
        ("mimetype", "application/epub+zip".to_string()),
        (
            "META-INF/container.xml",
            "<?xml version=\"1.0\"?><container version=\"1.0\" xmlns=\"urn:oasis:names:tc:opendocument:xmlns:container\"><rootfiles><rootfile full-path=\"OEBPS/content.opf\" media-type=\"application/oebps-package+xml\"/></rootfiles></container>".to_string(),
        ),
        (
            "OEBPS/content.opf",
            "<?xml version=\"1.0\"?><package xmlns=\"http://www.idpf.org/2007/opf\" version=\"2.0\"><metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\"><dc:title>Persistent LanternLeaf Fixture</dc:title><dc:language>en</dc:language></metadata><manifest><item id=\"chapter\" href=\"chapter.xhtml\" media-type=\"application/xhtml+xml\"/></manifest><spine><itemref idref=\"chapter\"/></spine></package>".to_string(),
        ),
        (
            "OEBPS/chapter.xhtml",
            format!("<html xmlns=\"http://www.w3.org/1999/xhtml\"><body><h1>Persistent fixture</h1>{paragraphs}</body></html>"),
        ),
    ];
    let mut bytes = Vec::new();
    let mut central = Vec::new();
    for (name, content) in &entries {
        let name_bytes = name.as_bytes();
        let data = content.as_bytes();
        let offset = bytes.len() as u32;
        let crc = crc32(data);
        bytes.extend_from_slice(&0x04034b50u32.to_le_bytes());
        bytes.extend_from_slice(&20u16.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&crc.to_le_bytes());
        bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(name_bytes);
        bytes.extend_from_slice(data);

        central.extend_from_slice(&0x02014b50u32.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&crc.to_le_bytes());
        central.extend_from_slice(&(data.len() as u32).to_le_bytes());
        central.extend_from_slice(&(data.len() as u32).to_le_bytes());
        central.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes());
        central.extend_from_slice(&0u32.to_le_bytes());
        central.extend_from_slice(&offset.to_le_bytes());
        central.extend_from_slice(name_bytes);
    }
    let central_offset = bytes.len() as u32;
    bytes.extend_from_slice(&central);
    bytes.extend_from_slice(&0x06054b50u32.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    bytes.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    bytes.extend_from_slice(&(central.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&central_offset.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    fs::write(path, bytes).expect("write EPUB fixture");
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for byte in data {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn cleanup_source(path: &Path) {
    let _ = cache::delete_recent_source_and_cache(path);
    let _ = fs::remove_file(path);
}

fn sample_housekeeping(path: &Path, config: config::AppConfig) -> ReaderHousekeeping {
    ReaderHousekeeping::from_parts(
        path.to_string_lossy().to_string(),
        cache::Bookmark {
            page: 0,
            sentence_idx: Some(0),
            sentence_text: Some("sentence".to_string()),
            scroll_y: 0.0,
            pdf_page_idx: None,
            pdf_rects: Vec::new(),
            pdf_line_rects: Vec::new(),
            pdf_block_rects: Vec::new(),
            pdf_confidence: None,
            pdf_reason: None,
            pdf_quality_class: None,
            pdf_sentence_text_hash: None,
            pdf_token_lineage: Vec::new(),
        },
        config,
        None,
    )
}

fn sample_snapshot(path: &Path) -> session::ReaderSnapshot {
    session::ReaderSnapshot {
        source_path: path.to_string_lossy().to_string(),
        source_name: path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("test.epub")
            .to_string(),
        current_page: 0,
        total_pages: 1,
        text_only_mode: false,
        has_structured_markdown: false,
        pretty_kind: session::PrettyKind::None,
        pdf_geometry_mode: None,
        pdf_sync_strategy: None,
        pdf_classification: None,
        pdf_runtime_policy: None,
        pdf_ocr_alignment: None,
        pdf_ocr_pipeline: None,
        images: Vec::new(),
        tts_text_page: "tts".to_string(),
        reading_markdown_page: None,
        reading_html_page: None,
        tts_current_sentence_text: None,
        page_text: "page".to_string(),
        sentences: vec!["sentence".to_string()],
        canonical_sentences: vec!["sentence".to_string()].into(),
        page_sentence_counts: vec![1].into(),
        sentence_anchor_map: vec![Some(0)].into(),
        structured_document: None,
        highlighted_canonical_idx: Some(0),
        highlighted_sentence_idx: Some(0),
        search_query: String::new(),
        search_matches: vec![0].into(),
        selected_search_match: Some(0),
        settings: session::ReaderSettingsView {
            theme: config::ThemeMode::Day,
            font_family: config::FontFamily::Lexend,
            font_weight: config::FontWeight::Normal,
            day_highlight: config::HighlightColor {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.4,
            },
            night_highlight: config::HighlightColor {
                r: 0.5,
                g: 0.6,
                b: 0.7,
                a: 0.8,
            },
            font_size: 18,
            line_spacing: 1.2,
            word_spacing: 0,
            letter_spacing: 0,
            margin_horizontal: 20,
            margin_vertical: 12,
            lines_per_page: 40,
            pause_after_sentence: 0.0,
            auto_scroll_tts: true,
            center_spoken_sentence: true,
            text_only_show_original_text: false,
            time_remaining_display: config::TimeRemainingDisplay::Adaptive,
            tts_speed: 1.0,
            tts_volume: 1.0,
            tts_backend: config::TtsBackend::Piper,
            windows_voice_id: None,
            pretty: config::PrettyUiConfig::default(),
        },
        tts: session::ReaderTtsView {
            state: session::TtsPlaybackState::Paused,
            current_sentence_idx: Some(0),
            sentence_count: 1,
            can_seek_prev: false,
            can_seek_next: false,
            progress_pct: 0.0,
        },
        stats: session::ReaderStats {
            page_index: 0,
            total_pages: 1,
            tts_progress_pct: 0.0,
            global_progress_pct: 0.0,
            page_time_remaining_secs: 0.0,
            book_time_remaining_secs: 0.0,
            page_word_count: 1,
            page_sentence_count: 1,
            page_start_percent: 0.0,
            page_end_percent: 1.0,
            words_read_up_to_page_start: 0,
            sentences_read_up_to_page_start: 0,
            words_read_up_to_page_end: 1,
            sentences_read_up_to_page_end: 1,
            words_read_up_to_current_position: 1,
            sentences_read_up_to_current_position: 1,
        },
        panels: session::PanelState {
            show_settings: false,
            show_stats: false,
            show_tts: true,
        },
        pdf_document_handle: None,
    }
}

#[test]
fn persistence_roundtrip_and_delete() {
    with_cache_root(|| {
        let source = unique_source_path("epub");
        write_source(&source);
        cache::remember_source_path(&source);
        let lifecycle =
            PersistenceLifecycle::new(Arc::new(FilesystemPersistenceService::default()));
        let config = config::AppConfig::default();

        lifecycle.flush_trigger(
            Some(sample_housekeeping(&source, config)),
            PersistenceTrigger::SourceOpen,
        );
        let loaded = cache::load_bookmark(&source);
        assert!(loaded.is_some(), "bookmark should be persisted");

        cache::delete_recent_source_and_cache(&source).expect("delete source and cache");
        let loaded_after_delete = cache::load_bookmark(&source);
        assert!(loaded_after_delete.is_none(), "bookmark should be deleted");

        cleanup_source(&source);
    });
}

#[test]
fn persistence_rebuilds_after_corruption() {
    with_cache_root(|| {
        let source = unique_source_path("epub");
        write_source(&source);
        cache::remember_source_path(&source);

        let bookmark_path = cache::hash_dir(&source).join("bookmark.toml");
        if let Some(parent) = bookmark_path.parent() {
            fs::create_dir_all(parent).expect("create cache dir");
        }
        fs::write(&bookmark_path, "not = valid = toml").expect("write corrupt bookmark");
        let loaded = cache::load_bookmark(&source);
        assert!(loaded.is_none(), "corrupt bookmark should be ignored");

        let lifecycle =
            PersistenceLifecycle::new(Arc::new(FilesystemPersistenceService::default()));
        let config = config::AppConfig::default();
        lifecycle.flush_trigger(
            Some(sample_housekeeping(&source, config)),
            PersistenceTrigger::SourceOpen,
        );
        let rebuilt = fs::read_to_string(&bookmark_path).unwrap_or_default();
        assert!(
            rebuilt.contains("page"),
            "bookmark file should be rebuilt with content"
        );

        cleanup_source(&source);
    });
}

#[test]
fn persisted_epub_close_reopen_rebuilds_incomplete_pretty_artifact_without_healing_commands() {
    with_cache_root(|| {
        let source = unique_source_path("epub");
        write_epub_source(&source);
        let normalizer = lanternleaf_core::normalizer::TextNormalizer::load_default();
        let config = config::AppConfig::default();
        let mut first = session::load_session_for_source(source.clone(), &config, &normalizer)
            .expect("load representative EPUB");
        let initial = first.snapshot(session::PanelState::default(), &normalizer);
        let initial_canonical_count = initial.canonical_sentences.len();
        let initial_html = initial.reading_html_page.clone().expect("Pretty HTML");
        assert!(initial_canonical_count > 128);
        assert!(initial_html.contains("Persistent EPUB paragraph 179"));

        for enabled in [true, false, true, false, true, false] {
            let _ = first.apply_command_lightweight(
                session::SessionCommand::SetTextOnly { enabled },
                &normalizer,
            );
        }
        let _ = first.apply_command_lightweight(
            session::SessionCommand::SentenceClick { sentence_idx: 7 },
            &normalizer,
        );
        let bookmark = first.to_bookmark();
        PersistenceLifecycle::new(Arc::new(FilesystemPersistenceService::default()))
            .on_session_close(ReaderHousekeeping::from_parts(
                source.to_string_lossy(),
                bookmark.clone(),
                config.clone(),
                None,
            ));
        let content_dir = cache::hash_dir(&source).join("content");
        assert!(content_dir.join("reading-html.html").is_file());
        fs::write(
            content_dir.join("reading-html.html"),
            "<p>truncated stale result</p>",
        )
        .expect("write deliberately incomplete derived artifact");
        drop(first);

        let mut reopened = session::load_session_for_source(source.clone(), &config, &normalizer)
            .expect("reopen same EPUB through production loader");
        let reopened_snapshot = reopened.snapshot(session::PanelState::default(), &normalizer);
        assert_eq!(
            reopened_snapshot.canonical_sentences.len(),
            initial_canonical_count
        );
        assert!(
            reopened_snapshot
                .reading_html_page
                .as_deref()
                .is_some_and(|html| html.contains("Persistent EPUB paragraph 179")),
            "incomplete derived Pretty content must not become authoritative"
        );
        assert_eq!(
            reopened_snapshot.highlighted_canonical_idx, bookmark.sentence_idx,
            "valid canonical bookmark must survive close/reopen"
        );
        assert_ne!(
            fs::read_to_string(content_dir.join("reading-html.html")).unwrap_or_default(),
            "<p>truncated stale result</p>",
            "the normal source load must rebuild the incomplete artifact"
        );

        cleanup_source(&source);
    });
}
