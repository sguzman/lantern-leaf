#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryFrom,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    io::{Read, Seek, SeekFrom},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex, OnceLock, Weak,
        mpsc::{self, Receiver},
    },
    thread::{self, JoinHandle},
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

    pub(crate) fn page_metadata(&self, source_path: &Path) -> Result<Vec<PdfPageDimension>, NativePdfRendererError> {
        let document = self.pdfium.load_pdf_from_file(source_path, None)?;
        let pages = document.pages();
        if pages.is_empty() {
            return Err(NativePdfRendererError::PageIndexOutOfBounds(0));
        }
        (0..pages.len())
            .map(|index| {
                let page = pages.get(index)?;
                Ok(PdfPageDimension {
                    width: page.width().value.abs().max(1.0),
                    height: page.height().value.abs().max(1.0),
                })
            })
            .collect()
    }

    pub(crate) fn page_count(&self, source_path: &Path) -> Result<usize, NativePdfRendererError> {
        Ok(self.page_metadata(source_path)?.len())
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

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PdfRenderSpec {
    pub source: PathBuf,
    pub generation: u64,
    pub width: u32,
    pub height: u32,
}

#[cfg(not(target_arch = "wasm32"))]
impl PdfRenderSpec {
    pub(crate) fn from_effective_zoom(
        source: PathBuf,
        generation: u64,
        effective_zoom: f32,
    ) -> Self {
        Self {
            source,
            generation,
            width: quantized_render_width(crate::pdf_viewport::PDF_BASE_PAGE_WIDTH, effective_zoom),
            height: PDF_RENDER_MAX_HEIGHT,
        }
    }

    pub(crate) fn key_for(&self, page_index: usize) -> PdfRenderKey {
        PdfRenderKey {
            source: self.source.clone(),
            generation: self.generation,
            page_index,
            width: self.width,
            height: self.height,
        }
    }
}

pub(crate) fn accepts_render_result(key: &PdfRenderKey, source: &Path, generation: u64) -> bool {
    key.generation == generation && key.source == source
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn accepts_render_result_for_spec(key: &PdfRenderKey, spec: &PdfRenderSpec) -> bool {
    key == &spec.key_for(key.page_index)
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

pub(crate) fn presented_page_size(
    texture_size: [usize; 2],
    viewport_width: f32,
    zoom: f32,
    max_presented_width: f32,
) -> [f32; 2] {
    let aspect = texture_size[1].max(1) as f32 / texture_size[0].max(1) as f32;
    let width = (viewport_width.max(1.0) * zoom.max(0.1)).min(max_presented_width.max(1.0));
    [width, (width * aspect).max(1.0)]
}

pub(crate) fn quantized_render_width(viewport_width: f32, zoom: f32) -> u32 {
    let requested =
        (viewport_width.max(320.0) * zoom.max(0.1)).clamp(320.0, PDF_RENDER_MAX_WIDTH as f32);
    ((requested / 64.0).ceil() as u32 * 64).min(PDF_RENDER_MAX_WIDTH)
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
pub(crate) struct PdfResidentEntry {
    pub key: PdfRenderKey,
    pub last_touched: u64,
    pub pinned: bool,
    pub keep: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn choose_resident_texture_evictions(
    mut entries: Vec<PdfResidentEntry>,
    capacity: usize,
) -> Vec<PdfRenderKey> {
    let overflow = entries.len().saturating_sub(capacity);
    if overflow == 0 {
        return Vec::new();
    }
    entries.sort_by_key(|entry| {
        (
            entry.pinned,
            entry.keep,
            entry.last_touched,
            entry.key.generation,
            entry.key.page_index,
            entry.key.width,
        )
    });
    entries
        .into_iter()
        .filter(|entry| !entry.pinned)
        .take(overflow)
        .map(|entry| entry.key)
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn resident_texture_evictions_for_surface(
    resident_keys: impl IntoIterator<Item = PdfRenderKey>,
    touches: &HashMap<PdfRenderKey, u64>,
    current_key: &PdfRenderKey,
    keep_pages: &[usize],
    capacity: usize,
) -> Vec<PdfRenderKey> {
    let entries = resident_keys
        .into_iter()
        .map(|key| PdfResidentEntry {
            pinned: key == *current_key,
            keep: key.source == current_key.source
                && key.generation == current_key.generation
                && key.width == current_key.width
                && keep_pages.contains(&key.page_index),
            last_touched: touches.get(&key).copied().unwrap_or_default(),
            key,
        })
        .collect();
    choose_resident_texture_evictions(entries, capacity)
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PdfRequestPriority {
    Nearby,
    Current,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
struct QueuedRender {
    key: PdfRenderKey,
    priority: PdfRequestPriority,
    sequence: u64,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct PdfRenderScheduler {
    queued: Vec<QueuedRender>,
    in_flight: HashSet<PdfRenderKey>,
    next_sequence: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl PdfRenderScheduler {
    fn submit(&mut self, key: PdfRenderKey, priority: PdfRequestPriority) -> bool {
        if self.in_flight.contains(&key) || self.queued.iter().any(|item| item.key == key) {
            return false;
        }
        if priority == PdfRequestPriority::Current {
            // A new viewport anchor supersedes obsolete queued work.  Keep only
            // the exact request being promoted; in-flight work remains bounded
            // and is rejected by generation/source identity when stale.
            self.queued.retain(|item| item.key == key);
        } else if self.queued.len() >= PDF_RENDER_QUEUE_CAPACITY {
            return false;
        }
        if self.queued.len() >= PDF_RENDER_QUEUE_CAPACITY {
            return false;
        }
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        self.queued.push(QueuedRender {
            key,
            priority,
            sequence,
        });
        true
    }

    fn take_next(&mut self) -> Option<PdfRenderKey> {
        let index = self
            .queued
            .iter()
            .enumerate()
            .max_by_key(|(_, item)| (item.priority, item.key.generation, item.sequence))
            .map(|(index, _)| index)?;
        let item = self.queued.remove(index);
        self.in_flight.insert(item.key.clone());
        Some(item.key)
    }

    fn finish(&mut self, key: &PdfRenderKey) {
        self.in_flight.remove(key);
    }

    fn pending_len(&self) -> usize {
        self.queued.len() + self.in_flight.len()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) struct PdfRenderResult {
    pub key: PdfRenderKey,
    pub image: Result<ColorImage, String>,
    pub worker_thread: thread::ThreadId,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PdfMetadata {
    pub page_count: usize,
    pub page_dimensions: Vec<PdfPageDimension>,
    pub worker_thread: thread::ThreadId,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PdfPageDimension {
    pub width: f32,
    pub height: f32,
}

#[cfg(not(target_arch = "wasm32"))]
struct PdfMetadataRequest {
    source: PathBuf,
    reply: mpsc::SyncSender<Result<PdfMetadata, String>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub(crate) struct PdfNativeService {
    scheduler: Arc<(Mutex<PdfRenderScheduler>, Condvar)>,
    metadata_tx: mpsc::SyncSender<PdfMetadataRequest>,
    result_rx: Arc<Mutex<Receiver<PdfRenderResult>>>,
    lifetime: Arc<()>,
    shutdown: Arc<AtomicBool>,
    worker: Arc<Mutex<Option<JoinHandle<()>>>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl PdfNativeService {
    #[cfg(test)]
    pub(crate) fn test_stub() -> Self {
        Self {
            scheduler: Arc::new((Mutex::new(PdfRenderScheduler::default()), Condvar::new())),
            metadata_tx: mpsc::sync_channel(1).0,
            result_rx: Arc::new(Mutex::new(mpsc::sync_channel(1).1)),
            lifetime: Arc::new(()),
            shutdown: Arc::new(AtomicBool::new(true)),
            worker: Arc::new(Mutex::new(None)),
        }
    }

    pub(crate) fn start() -> Self {
        #[cfg(test)]
        return Self::start_owned();

        #[cfg(not(test))]
        {
        static SERVICE: OnceLock<PdfNativeService> = OnceLock::new();
            SERVICE.get_or_init(Self::start_owned).clone()
        }
    }

    fn start_owned() -> Self {
        let scheduler = Arc::new((Mutex::new(PdfRenderScheduler::default()), Condvar::new()));
        let worker_scheduler = Arc::clone(&scheduler);
        let lifetime = Arc::new(());
        let worker_lifetime = Arc::downgrade(&lifetime);
        let shutdown = Arc::new(AtomicBool::new(false));
        let worker_shutdown = Arc::clone(&shutdown);
        let (metadata_tx, metadata_rx) = mpsc::sync_channel::<PdfMetadataRequest>(2);
        let (result_tx, result_rx) = mpsc::sync_channel(PDF_RENDER_QUEUE_CAPACITY);
        let worker_handle = thread::Builder::new()
            .name("lanternleaf-pdf-native".to_string())
            .spawn(move || {
                let mut renderer = NativePdfRenderer::new().ok();
                let worker_thread = thread::current().id();
                loop {
                    if worker_lifetime.upgrade().is_none()
                        || worker_shutdown.load(Ordering::Acquire)
                    {
                        break;
                    }
                    if let Ok(request) = metadata_rx.try_recv() {
                        let result = match renderer.as_ref() {
                            Some(renderer) => validate_pdf_source(&request.source)
                                .and_then(|()| {
                                    renderer.page_metadata(&request.source).map_err(|err| format!("{err:?}"))
                                })
                                .map(|page_dimensions| PdfMetadata {
                                    page_count: page_dimensions.len(),
                                    page_dimensions,
                                    worker_thread,
                                }),
                            None => Err("native PDF service is unavailable".to_string()),
                        };
                        let _ = request.reply.send(result);
                        continue;
                    }

                    let key = {
                        let (lock, wake) = &*worker_scheduler;
                        let mut scheduler = lock.lock().expect("PDF scheduler lock");
                        loop {
                            if worker_lifetime.upgrade().is_none()
                                || worker_shutdown.load(Ordering::Acquire)
                            {
                                return;
                            }
                            if let Some(key) = scheduler.take_next() {
                                break Some(key);
                            }
                            let (next_scheduler, _) = wake
                                .wait_timeout(scheduler, Duration::from_millis(10))
                                .expect("PDF scheduler wait");
                            scheduler = next_scheduler;
                            break None;
                        }
                    };
                    let Some(key) = key else {
                        continue;
                    };
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
                        None => Err("native PDF service is unavailable".to_string()),
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
            .expect("PDF native service must start");
        Self {
            scheduler,
            metadata_tx,
            result_rx: Arc::new(Mutex::new(result_rx)),
            lifetime,
            shutdown,
            worker: Arc::new(Mutex::new(Some(worker_handle))),
        }
    }

    pub(crate) fn metadata(&self, source: PathBuf) -> Result<PdfMetadata, String> {
        let (reply_tx, reply_rx) = mpsc::sync_channel(1);
        self.metadata_tx
            .send(PdfMetadataRequest {
                source,
                reply: reply_tx,
            })
            .map_err(|_| "native PDF service stopped".to_string())?;
        self.scheduler.1.notify_one();
        reply_rx
            .recv()
            .map_err(|_| "native PDF metadata request stopped".to_string())?
    }

    pub(crate) fn metadata_async(&self, source: PathBuf) -> mpsc::Receiver<Result<PdfMetadata, String>> {
        let (reply_tx, reply_rx) = mpsc::sync_channel(1);
        if self.metadata_tx.send(PdfMetadataRequest { source, reply: reply_tx }).is_ok() {
            self.scheduler.1.notify_one();
        }
        reply_rx
    }

    fn submit_render(&self, key: PdfRenderKey, priority: PdfRequestPriority) -> bool {
        let (lock, wake) = &*self.scheduler;
        let submitted = lock
            .lock()
            .expect("PDF scheduler lock")
            .submit(key, priority);
        if submitted {
            wake.notify_one();
        }
        submitted
    }

    fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Release);
        self.scheduler.1.notify_all();
        if let Some(worker) = self.worker.lock().expect("PDF worker lock").take() {
            let _ = worker.join();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for PdfNativeService {
    fn drop(&mut self) {
        if Arc::strong_count(&self.lifetime) == 1 {
            self.shutdown();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_pdf_source(source: &Path) -> Result<(), String> {
    const PDF_TAIL_CHECK_BYTES: u64 = 64 * 1024;
    let mut file = std::fs::File::open(source).map_err(|err| err.to_string())?;
    let mut header = [0u8; 5];
    file.read_exact(&mut header).map_err(|err| err.to_string())?;
    let length = file.metadata().map_err(|err| err.to_string())?.len();
    let tail_length = length.min(PDF_TAIL_CHECK_BYTES);
    file.seek(SeekFrom::End(-(tail_length as i64)))
        .map_err(|err| err.to_string())?;
    let mut tail = vec![0u8; tail_length as usize];
    file.read_exact(&mut tail).map_err(|err| err.to_string())?;
    if &header != b"%PDF-" || !tail.windows(b"%%EOF".len()).any(|window| window == b"%%EOF") {
        return Err("source is not a complete PDF document".to_string());
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct PdfRenderWorker {
    service: PdfNativeService,
}

#[cfg(not(target_arch = "wasm32"))]
impl PdfRenderWorker {
    pub(crate) fn start() -> (Self, PdfNativeService) {
        #[cfg(test)]
        {
            let service = PdfNativeService::test_stub();
            return (Self { service: service.clone() }, service);
        }

        #[cfg(not(test))]
        {
        let service = PdfNativeService::start();
            (Self { service: service.clone() }, service)
        }
    }

    pub(crate) fn from_service(service: PdfNativeService) -> Self {
        Self { service }
    }

    pub(crate) fn request(&mut self, key: PdfRenderKey, priority: PdfRequestPriority) -> bool {
        self.service.submit_render(key, priority)
    }

    pub(crate) fn request_metadata(&self, source: PathBuf) -> mpsc::Receiver<Result<PdfMetadata, String>> {
        self.service.metadata_async(source)
    }

    pub(crate) fn drain(&mut self) -> Vec<PdfRenderResult> {
        let mut results = Vec::new();
        let result_rx = self.service.result_rx.lock().expect("PDF result lock");
        while let Ok(result) = result_rx.try_recv() {
            self.service
                .scheduler
                .0
                .lock()
                .expect("PDF scheduler lock")
                .finish(&result.key);
            results.push(result);
        }
        results
    }

    pub(crate) fn pending_len(&self) -> usize {
        self.service
            .scheduler
            .0
            .lock()
            .expect("PDF scheduler lock")
            .pending_len()
    }

    #[cfg(test)]
    fn shutdown(&self) {
        self.service.shutdown();
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    fn two_page_pdf_bytes() -> Vec<u8> {
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>",
            "<< /Type /Pages /Kids [3 0 R 4 0 R] /Count 2 >>",
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << >> /Contents 5 0 R >>",
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << >> /Contents 5 0 R >>",
            "<< /Length 0 >>\nstream\n\nendstream",
        ];
        let mut bytes = b"%PDF-1.4\n".to_vec();
        let mut offsets = vec![0usize];
        for (index, object) in objects.iter().enumerate() {
            offsets.push(bytes.len());
            bytes
                .extend_from_slice(format!("{} 0 obj\n{}\nendobj\n", index + 1, object).as_bytes());
        }
        let xref_offset = bytes.len();
        bytes.extend_from_slice(format!("xref\n0 {}\n", offsets.len()).as_bytes());
        bytes.extend_from_slice(b"0000000000 65535 f \n");
        for offset in offsets.iter().skip(1) {
            bytes.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        bytes.extend_from_slice(
            format!(
                "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
                offsets.len(),
                xref_offset
            )
            .as_bytes(),
        );
        bytes
    }

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
    fn canonical_render_spec_matches_scheduler_and_presentation_keys() {
        let source = PathBuf::from("book.pdf");
        let modes = [
            crate::pdf_subsystem::PdfZoomState::manual(1.0),
            crate::pdf_subsystem::PdfZoomState::manual(3.0),
            crate::pdf_subsystem::PdfZoomState {
                mode: crate::pdf_subsystem::PdfZoomMode::FitWidth,
                manual_level: 1.0,
            },
            crate::pdf_subsystem::PdfZoomState {
                mode: crate::pdf_subsystem::PdfZoomMode::FitPage,
                manual_level: 1.0,
            },
        ];
        for state in modes {
            let effective = state.effective_level(1200.0, 700.0, 816.0, 1154.0);
            let spec = PdfRenderSpec::from_effective_zoom(source.clone(), 9, effective);
            let scheduled = spec.key_for(7);
            let presentation = spec.key_for(7);
            assert_eq!(scheduled, presentation);
            assert_eq!(scheduled.width, quantized_render_width(816.0, effective));
            assert_eq!(scheduled.height, PDF_RENDER_MAX_HEIGHT);
        }
    }

    #[test]
    fn completed_authoritative_raster_is_discoverable_by_presentation_key() {
        let spec = PdfRenderSpec::from_effective_zoom(PathBuf::from("book.pdf"), 12, 2.0);
        let completed = spec.key_for(11);
        let mut textures = HashMap::new();
        textures.insert(completed.clone(), 1u8);
        assert!(textures.contains_key(&spec.key_for(11)));
        assert!(!textures.contains_key(&spec.key_for(12)));
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
        let spec = PdfRenderSpec::from_effective_zoom(PathBuf::from("book.pdf"), 3, 1.0);
        assert!(accepts_render_result_for_spec(&spec.key_for(0), &spec));
        assert!(!accepts_render_result_for_spec(&key(3, 0, 800), &spec));
    }

    #[test]
    fn bounded_pages_always_preserve_current_page() {
        let pages = bounded_render_pages([8, 2, 8, 4], 4, 3);
        assert_eq!(pages, vec![4, 8, 2]);
        assert_eq!(bounded_render_pages([1, 2], 7, 0), Vec::<usize>::new());
    }

    #[test]
    fn presentation_zoom_changes_logical_size_without_changing_aspect() {
        let fit = presented_page_size([1000, 1400], 800.0, 1.0, 4800.0);
        let zoomed = presented_page_size([1000, 1400], 800.0, 1.5, 4800.0);
        assert_eq!(fit, [800.0, 1120.0]);
        assert_eq!(zoomed, [1200.0, 1680.0]);
        assert!(zoomed[0] > 800.0);
    }

    #[test]
    fn landscape_presentation_preserves_landscape_aspect() {
        let size = presented_page_size([1600, 900], 800.0, 1.0, 4800.0);
        assert_eq!(size, [800.0, 450.0]);
        assert!(size[0] > size[1]);
    }

    #[test]
    fn navigation_owns_a_new_visible_page_key() {
        assert_ne!(key(4, 1, 832), key(4, 2, 832));
    }

    #[test]
    fn raster_width_is_quantized_and_bounded() {
        assert_eq!(quantized_render_width(801.0, 1.0), 832);
        assert_eq!(quantized_render_width(10_000.0, 2.0), PDF_RENDER_MAX_WIDTH);
    }

    #[test]
    fn resident_eviction_pins_current_and_prefers_irrelevant_old_pages() {
        let entries = (0..4)
            .map(|page| PdfResidentEntry {
                key: key(1, page, 800),
                last_touched: page as u64,
                pinned: page == 2,
                keep: page == 1 || page == 2,
            })
            .collect();
        let evicted = choose_resident_texture_evictions(entries, 2);
        assert_eq!(evicted, vec![key(1, 0, 800), key(1, 3, 800)]);
    }

    #[test]
    fn production_surface_residency_path_keeps_current_over_capacity() {
        let current = key(7, 7, 832);
        let mut resident = HashMap::new();
        for page in 0..12 {
            resident.insert(key(7, page, 832), page as u64);
        }
        let evicted = resident_texture_evictions_for_surface(
            resident.keys().cloned(),
            &resident,
            &current,
            &[current.page_index],
            8,
        );
        for key in &evicted {
            resident.remove(key);
        }
        assert!(resident.len() <= 8);
        assert!(resident.contains_key(&current));
        assert!(evicted.iter().all(|key| key != &current));
        assert!(evicted.iter().any(|key| key.page_index == 0));
    }

    #[test]
    fn scheduler_coalesces_and_prioritizes_new_current_work() {
        let mut scheduler = PdfRenderScheduler::default();
        assert!(scheduler.submit(key(1, 1, 800), PdfRequestPriority::Nearby));
        assert!(scheduler.submit(key(1, 2, 800), PdfRequestPriority::Nearby));
        assert!(scheduler.submit(key(2, 7, 1200), PdfRequestPriority::Current));
        assert_eq!(scheduler.pending_len(), 1);
        assert_eq!(scheduler.take_next(), Some(key(2, 7, 1200)));
        assert!(!scheduler.submit(key(2, 7, 1200), PdfRequestPriority::Current));
    }

    #[test]
    fn scheduler_rejects_duplicate_frame_requests() {
        let mut scheduler = PdfRenderScheduler::default();
        assert!(scheduler.submit(key(1, 1, 800), PdfRequestPriority::Nearby));
        assert!(!scheduler.submit(key(1, 1, 800), PdfRequestPriority::Nearby));
        assert_eq!(scheduler.pending_len(), 1);
    }

    #[test]
    fn finished_failure_terminalizes_request_and_allows_recovery() {
        let mut scheduler = PdfRenderScheduler::default();
        let failed = key(1, 3, 800);
        assert!(scheduler.submit(failed.clone(), PdfRequestPriority::Current));
        assert_eq!(scheduler.take_next(), Some(failed.clone()));
        scheduler.finish(&failed);
        assert_eq!(scheduler.pending_len(), 0);
        assert!(scheduler.submit(failed, PdfRequestPriority::Current));
    }

    #[test]
    #[ignore = "run as a dedicated process-wide Pdfium lifecycle probe"]
    fn production_service_uses_one_owner_for_metadata_and_raster_and_rejects_malformed_input() {
        let root =
            std::env::temp_dir().join(format!("lanternleaf-pdf-probe-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("probe temp directory");
        let valid = root.join("two-pages.pdf");
        let invalid = root.join("header-only.pdf");
        std::fs::write(&valid, two_page_pdf_bytes()).expect("valid PDF");
        std::fs::write(&invalid, b"%PDF-1.7\n").expect("header-only PDF");

        let service = PdfNativeService::start();
        thread::sleep(Duration::from_millis(50));
        let metadata = service.metadata(valid.clone()).expect("native page count");
        assert_eq!(metadata.page_count, 2);
        assert_eq!(metadata.page_dimensions.len(), 2);
        assert!(metadata.page_dimensions.iter().all(|dimension| dimension.width > 0.0 && dimension.height > 0.0));
        assert!(service.metadata(invalid).is_err());

        let mut worker = PdfRenderWorker::from_service(service);
        let render_key = PdfRenderKey {
            source: valid,
            generation: 1,
            page_index: 0,
            width: 832,
            height: 1200,
        };
        assert!(worker.request(render_key.clone(), PdfRequestPriority::Current));
        let result = (0..200).find_map(|_| {
            let result = worker
                .drain()
                .into_iter()
                .find(|result| result.key == render_key);
            if result.is_none() {
                thread::sleep(Duration::from_millis(10));
            }
            result
        });
        let result = result.expect("shared native service render result");
        assert!(result.image.is_ok());
        assert_eq!(metadata.worker_thread, result.worker_thread);

        thread::sleep(Duration::from_millis(50));
        let second_metadata = worker
            .service
            .metadata(root.join("two-pages.pdf"))
            .expect("second idle metadata request");
        assert_eq!(second_metadata.page_count, 2);
        assert_eq!(metadata.worker_thread, second_metadata.worker_thread);
        worker.shutdown();

        let _ = std::fs::remove_dir_all(root);
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
