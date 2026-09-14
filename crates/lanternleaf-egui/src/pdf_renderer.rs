#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, SyncSender, TrySendError},
    thread,
    time::{Duration, Instant},
};

#[cfg(not(target_arch = "wasm32"))]
use crate::constants::{PDF_CANVAS_TEXTURE_SIZE, PDF_TEXT_TEXTURE_SIZE};
#[cfg(not(target_arch = "wasm32"))]
use eframe::egui::ColorImage;
#[cfg(not(target_arch = "wasm32"))]
use pdfium_auto::bind_bundled;
#[cfg(not(target_arch = "wasm32"))]
use pdfium_render::prelude::*;

const CACHE_CAPACITY: usize = 12;
pub(crate) const PDF_RENDER_QUEUE_CAPACITY: usize = 8;
pub(crate) const PDF_RENDER_MAX_WIDTH: u32 = 2400;
pub(crate) const PDF_RENDER_MAX_HEIGHT: u32 = 3600;

#[allow(dead_code)]
#[derive(Debug)]
pub enum NativePdfRendererError {
    Auto(pdfium_auto::PdfiumAutoError),
    Pdfium(PdfiumError),
    PageIndexOutOfBounds(usize),
}

impl From<pdfium_auto::PdfiumAutoError> for NativePdfRendererError {
    fn from(value: pdfium_auto::PdfiumAutoError) -> Self {
        NativePdfRendererError::Auto(value)
    }
}

impl From<PdfiumError> for NativePdfRendererError {
    fn from(value: PdfiumError) -> Self {
        NativePdfRendererError::Pdfium(value)
    }
}

pub struct NativePdfRenderer {
    pdfium: Pdfium,
    cache: HashMap<RenderCacheKey, ColorImage>,
    order: VecDeque<RenderCacheKey>,
    capacity: usize,
    eviction_events: Vec<NativeRenderEviction>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativePdfRenderer {
    pub fn new() -> Result<Self, NativePdfRendererError> {
        let pdfium = bind_bundled()?;
        Ok(Self {
            pdfium,
            cache: HashMap::new(),
            order: VecDeque::new(),
            capacity: CACHE_CAPACITY,
            eviction_events: Vec::new(),
        })
    }

    pub fn render_canvas(
        &mut self,
        source_path: &Path,
        page_index: usize,
    ) -> Result<RenderOutcome, NativePdfRendererError> {
        self.render_for_target(source_path, page_index, RenderTarget::Canvas)
    }

    pub fn render_canvas_at_size(
        &mut self,
        source_path: &Path,
        page_index: usize,
        max_width: u32,
        max_height: u32,
    ) -> Result<RenderOutcome, NativePdfRendererError> {
        self.render_for_dimensions(
            source_path,
            page_index,
            RenderTarget::Canvas,
            max_width,
            max_height,
        )
    }

    pub fn render_text_layer(
        &mut self,
        source_path: &Path,
        page_index: usize,
    ) -> Result<RenderOutcome, NativePdfRendererError> {
        self.render_for_target(source_path, page_index, RenderTarget::TextLayer)
    }

    fn render_for_target(
        &mut self,
        source_path: &Path,
        page_index: usize,
        target: RenderTarget,
    ) -> Result<RenderOutcome, NativePdfRendererError> {
        let key = RenderCacheKey {
            source: source_path.to_path_buf(),
            page_index,
            target,
            width: 0,
            height: 0,
        };
        self.render_for_dimensions(source_path, page_index, target, key.width, key.height)
    }

    fn render_for_dimensions(
        &mut self,
        source_path: &Path,
        page_index: usize,
        target: RenderTarget,
        max_width: u32,
        max_height: u32,
    ) -> Result<RenderOutcome, NativePdfRendererError> {
        let key = RenderCacheKey {
            source: source_path.to_path_buf(),
            page_index,
            target,
            width: max_width,
            height: max_height,
        };
        if let Some(image) = self.cache.get(&key) {
            return Ok(RenderOutcome {
                image: image.clone(),
                duration: Duration::from_micros(0),
                cache_hit: true,
            });
        }

        let start = Instant::now();
        let (image, duration) = {
            let document = self.pdfium.load_pdf_from_file(source_path, None)?;
            let page_index = PdfPageIndex::try_from(page_index)
                .map_err(|_| NativePdfRendererError::PageIndexOutOfBounds(page_index))?;
            let page = document.pages().get(page_index)?;
            let dims = if max_width == 0 || max_height == 0 {
                target.render_dimensions(&page)
            } else {
                target.render_dimensions_for(&page, max_width, max_height)
            };
            let config = PdfRenderConfig::new()
                .set_target_width(dims.width)
                .set_target_height(dims.height);

            let bitmap = page.render_with_config(&config)?;
            (Self::bitmap_to_color_image(bitmap)?, start.elapsed())
        };
        self.insert_cache(key.clone(), image.clone());
        Ok(RenderOutcome {
            image,
            duration,
            cache_hit: false,
        })
    }

    fn insert_cache(&mut self, key: RenderCacheKey, image: ColorImage) {
        if self.cache.contains_key(&key) {
            self.order.retain(|existing| existing != &key);
        }
        while self.cache.len() >= self.capacity {
            if let Some(old_key) = self.order.pop_front() {
                self.cache.remove(&old_key);
                self.eviction_events.push(NativeRenderEviction {
                    timestamp: Instant::now(),
                    target: old_key.target,
                    page_index: old_key.page_index,
                    reason: "capacity_evicted".to_string(),
                });
            }
        }
        self.order.push_back(key.clone());
        self.cache.insert(key, image);
    }

    fn bitmap_to_color_image(bitmap: PdfBitmap<'_>) -> Result<ColorImage, NativePdfRendererError> {
        let rgba = bitmap.as_image().into_rgba8();
        let width = rgba.width() as usize;
        let height = rgba.height() as usize;
        let raw = rgba.into_raw();
        Ok(ColorImage::from_rgba_unmultiplied([width, height], &raw))
    }

    pub fn drain_eviction_events(&mut self) -> Vec<NativeRenderEviction> {
        std::mem::take(&mut self.eviction_events)
    }
}

#[cfg(target_arch = "wasm32")]
impl NativePdfRenderer {
    pub fn new() -> Result<Self, NativePdfRendererError> {
        Err(NativePdfRendererError::PageIndexOutOfBounds(0)) // Dummy
    }

    pub fn render_canvas(
        &mut self,
        _source_path: &Path,
        _page_index: usize,
    ) -> Result<RenderOutcome, NativePdfRendererError> {
        Err(NativePdfRendererError::PageIndexOutOfBounds(0))
    }

    pub fn render_text_layer(
        &mut self,
        _source_path: &Path,
        _page_index: usize,
    ) -> Result<RenderOutcome, NativePdfRendererError> {
        Err(NativePdfRendererError::PageIndexOutOfBounds(0))
    }

    pub fn drain_eviction_events(&mut self) -> Vec<NativeRenderEviction> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum RenderTarget {
    Canvas,
    TextLayer,
}

impl RenderTarget {
    #[cfg(not(target_arch = "wasm32"))]
    fn render_dimensions(&self, page: &PdfPage) -> RenderDimensions {
        let max_size = match self {
            RenderTarget::Canvas => PDF_CANVAS_TEXTURE_SIZE,
            RenderTarget::TextLayer => PDF_TEXT_TEXTURE_SIZE,
        };
        let max_width = max_size[0] as f32;
        let max_height = max_size[1] as f32;
        let page_width = page.width().value.abs().max(1.0);
        let page_height = page.height().value.abs().max(1.0);
        let mut width = max_width;
        let mut height = (width * (page_height / page_width)).round();
        if height > max_height {
            height = max_height;
            width = (height * (page_width / page_height)).round();
        }
        let width = width.max(1.0).round() as Pixels;
        let height = height.max(1.0).round() as Pixels;
        RenderDimensions { width, height }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_dimensions_for(
        &self,
        page: &PdfPage,
        max_width: u32,
        max_height: u32,
    ) -> RenderDimensions {
        let width = f32::min(max_width.max(1) as f32, PDF_RENDER_MAX_WIDTH as f32);
        let height = f32::min(max_height.max(1) as f32, PDF_RENDER_MAX_HEIGHT as f32);
        let page_width = page.width().value.abs().max(1.0);
        let page_height = page.height().value.abs().max(1.0);
        let scale = (width / page_width).min(height / page_height);
        RenderDimensions {
            width: (page_width * scale).round().max(1.0) as Pixels,
            height: (page_height * scale).round().max(1.0) as Pixels,
        }
    }

    pub(crate) fn label(&self) -> &'static str {
        match self {
            RenderTarget::Canvas => "canvas",
            RenderTarget::TextLayer => "text-layer",
        }
    }
}

pub struct RenderOutcome {
    #[cfg(not(target_arch = "wasm32"))]
    pub image: ColorImage,
    #[cfg(target_arch = "wasm32")]
    pub image: (),
    pub duration: Duration,
    pub cache_hit: bool,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PdfRenderKey {
    pub source: PathBuf,
    pub generation: u64,
    pub page_index: usize,
    pub width: u32,
    pub height: u32,
}

pub(crate) fn accepts_render_result(key: &PdfRenderKey, source: &Path, generation: u64) -> bool {
    key.generation == generation && key.source == source
}

pub(crate) fn bounded_render_pages(
    pages: impl IntoIterator<Item = usize>,
    current_page: usize,
    limit: usize,
) -> Vec<usize> {
    let mut result = Vec::with_capacity(limit.min(1));
    if limit == 0 {
        return result;
    }
    result.push(current_page);
    for page in pages {
        if result.len() >= limit {
            break;
        }
        if !result.contains(&page) {
            result.push(page);
        }
    }
    result
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) struct PdfRenderResult {
    pub key: PdfRenderKey,
    pub image: Result<ColorImage, String>,
    pub worker_thread: thread::ThreadId,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct PdfRenderWorker {
    request_tx: SyncSender<PdfRenderKey>,
    result_rx: Receiver<PdfRenderResult>,
    pending: HashMap<PdfRenderKey, ()>,
}

#[cfg(not(target_arch = "wasm32"))]
impl PdfRenderWorker {
    pub(crate) fn start() -> Self {
        let (request_tx, request_rx) =
            mpsc::sync_channel::<PdfRenderKey>(PDF_RENDER_QUEUE_CAPACITY);
        let (result_tx, result_rx) = mpsc::sync_channel(PDF_RENDER_QUEUE_CAPACITY);
        thread::Builder::new()
            .name("lanternleaf-pdf-render".to_string())
            .spawn(move || {
                let mut renderer = NativePdfRenderer::new().ok();
                let worker_thread = thread::current().id();
                while let Ok(key) = request_rx.recv() {
                    let image = match renderer.as_mut() {
                        Some(renderer) => renderer
                            .render_canvas_at_size(
                                &key.source,
                                key.page_index,
                                key.width,
                                key.height,
                            )
                            .map(|outcome| outcome.image)
                            .map_err(|err| format!("{err:?}")),
                        None => Err("native PDF renderer is unavailable".to_string()),
                    };
                    if result_tx
                        .send(PdfRenderResult {
                            key,
                            image,
                            worker_thread,
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            })
            .expect("PDF render worker must start");
        Self {
            request_tx,
            result_rx,
            pending: HashMap::new(),
        }
    }

    pub(crate) fn request(&mut self, key: PdfRenderKey) -> bool {
        if self.pending.contains_key(&key) {
            return false;
        }
        match self.request_tx.try_send(key.clone()) {
            Ok(()) => {
                self.pending.insert(key, ());
                true
            }
            Err(TrySendError::Full(_)) | Err(TrySendError::Disconnected(_)) => false,
        }
    }

    pub(crate) fn drain(&mut self) -> Vec<PdfRenderResult> {
        let mut results = Vec::new();
        while let Ok(result) = self.result_rx.try_recv() {
            self.pending.remove(&result.key);
            results.push(result);
        }
        results
    }

    pub(crate) fn pending_len(&self) -> usize {
        self.pending.len()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    fn key(generation: u64, page_index: usize, width: u32) -> PdfRenderKey {
        PdfRenderKey {
            source: PathBuf::from("book.pdf"),
            generation,
            page_index,
            width,
            height: 3600,
        }
    }

    #[test]
    fn render_key_includes_page_and_quantized_size() {
        assert_ne!(key(1, 0, 800), key(1, 1, 800));
        assert_ne!(key(1, 0, 800), key(1, 0, 1200));
    }

    #[test]
    fn stale_source_and_generation_results_are_rejected() {
        assert!(accepts_render_result(
            &key(3, 0, 800),
            Path::new("book.pdf"),
            3
        ));
        assert!(!accepts_render_result(
            &key(2, 0, 800),
            Path::new("book.pdf"),
            3
        ));
        assert!(!accepts_render_result(
            &key(3, 0, 800),
            Path::new("other.pdf"),
            3
        ));
    }

    #[test]
    fn bounded_pages_always_preserve_current_page() {
        let pages = bounded_render_pages([8, 2, 8, 4], 4, 3);
        assert_eq!(pages, vec![4, 8, 2]);
        assert_eq!(bounded_render_pages([1, 2], 7, 0), Vec::<usize>::new());
    }

    #[test]
    fn worker_request_queue_is_bounded_by_channel_capacity() {
        let (tx, rx) = mpsc::sync_channel::<PdfRenderKey>(PDF_RENDER_QUEUE_CAPACITY);
        for index in 0..PDF_RENDER_QUEUE_CAPACITY {
            tx.try_send(key(1, index, 800)).expect("queue has capacity");
        }
        assert!(matches!(
            tx.try_send(key(1, 99, 800)),
            Err(TrySendError::Full(_))
        ));
        drop(rx);
    }
}

#[derive(Clone, Debug)]
pub struct NativeRenderSpan {
    pub timestamp: Instant,
    pub target: RenderTarget,
    pub page_index: usize,
    pub duration: Duration,
    pub cache_hit: bool,
}

impl NativeRenderSpan {
    pub fn describe(&self) -> String {
        format!(
            "Native render: page {} {} (cache hit: {}) {:.2?}",
            self.page_index + 1,
            self.target.label(),
            self.cache_hit,
            self.duration,
        )
    }
}

#[derive(Clone, Debug)]
pub struct NativeRenderEviction {
    pub timestamp: Instant,
    pub target: RenderTarget,
    pub page_index: usize,
    pub reason: String,
}

impl NativeRenderEviction {
    pub fn describe(&self) -> String {
        format!(
            "Evicted page {} {} ({})",
            self.page_index + 1,
            self.target.label(),
            self.reason
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RenderCacheKey {
    source: PathBuf,
    page_index: usize,
    target: RenderTarget,
    width: u32,
    height: u32,
}

impl Hash for RenderCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.page_index.hash(state);
        self.target.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Copy, Clone, Debug)]
struct RenderDimensions {
    width: Pixels,
    height: Pixels,
}
