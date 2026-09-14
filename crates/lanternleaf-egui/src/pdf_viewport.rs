//! Deterministic, lightweight geometry for the continuous native PDF surface.
//!
//! This module deliberately knows nothing about Pdfium or textures.  It lets the
//! egui thread reserve stable page slots and select a bounded render window while
//! native raster work remains owned by the worker.

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PdfPageGeometry {
    pub aspect_ratio: f32,
}

impl PdfPageGeometry {
    pub(crate) fn new(aspect_ratio: f32) -> Self {
        Self {
            aspect_ratio: aspect_ratio.clamp(0.1, 10.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PdfPageSlot {
    pub index: usize,
    pub top: f32,
    pub height: f32,
    pub width: f32,
}

impl PdfPageSlot {
    pub fn bottom(self) -> f32 {
        self.top + self.height
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PdfViewportGeometry {
    pages: Vec<PdfPageGeometry>,
    page_width: f32,
    gap: f32,
    slots: Vec<PdfPageSlot>,
    total_height: f32,
}

impl PdfViewportGeometry {
    pub(crate) fn new(
        page_sizes: &[PdfPageGeometry],
        page_width: f32,
        gap: f32,
    ) -> Self {
        let page_width = page_width.max(1.0);
        let gap = gap.max(0.0);
        let mut top = 0.0;
        let mut slots = Vec::with_capacity(page_sizes.len());
        for (index, page) in page_sizes.iter().enumerate() {
            let height = (page_width / page.aspect_ratio).max(1.0);
            slots.push(PdfPageSlot { index, top, height, width: page_width });
            top += height + gap;
        }
        let total_height = (top - gap).max(0.0);
        Self { pages: page_sizes.to_vec(), page_width, gap, slots, total_height }
    }

    pub(crate) fn total_height(&self) -> f32 { self.total_height }
    pub(crate) fn page_width(&self) -> f32 { self.page_width }
    pub(crate) fn slots(&self) -> &[PdfPageSlot] { &self.slots }

    pub(crate) fn visible_page_indexes(&self, top: f32, bottom: f32, overscan: f32) -> Vec<usize> {
        let top = top.max(0.0) - overscan.max(0.0);
        let bottom = bottom.max(top) + overscan.max(0.0);
        self.slots.iter()
            .filter(|slot| slot.bottom() >= top && slot.top <= bottom)
            .map(|slot| slot.index)
            .collect()
    }

    /// The page containing the viewport anchor.  A small hysteresis band around
    /// the midpoint prevents a one-pixel boundary oscillation while scrolling.
    pub(crate) fn current_page(&self, viewport_top: f32, viewport_height: f32, previous: usize) -> Option<usize> {
        if self.slots.is_empty() { return None; }
        let anchor = viewport_top.max(0.0) + viewport_height.max(1.0) * 0.5;
        let candidate = self.slots.iter().position(|slot| anchor <= slot.bottom())
            .unwrap_or(self.slots.len() - 1);
        let previous = previous.min(self.slots.len() - 1);
        let prior = self.slots[previous];
        if candidate != previous && anchor >= prior.top && anchor <= prior.bottom() {
            Some(previous)
        } else {
            Some(candidate)
        }
    }

    pub(crate) fn jump_offset(&self, page: usize, viewport_height: f32, anchor: f32) -> f32 {
        let slot = self.slots.get(page.min(self.slots.len().saturating_sub(1)));
        slot.map(|slot| (slot.top - viewport_height.max(0.0) * anchor.clamp(0.0, 1.0)).max(0.0)).unwrap_or(0.0)
    }

    pub(crate) fn page(&self, index: usize) -> Option<PdfPageGeometry> { self.pages.get(index).copied() }
    pub(crate) fn gap(&self) -> f32 { self.gap }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn geometry() -> PdfViewportGeometry {
        PdfViewportGeometry::new(&[PdfPageGeometry::new(1.0); 4], 400.0, 16.0)
    }

    #[test]
    fn bounded_visible_window_and_two_page_boundary() {
        let geometry = geometry();
        assert_eq!(geometry.visible_page_indexes(390.0, 430.0, 0.0), vec![0, 1]);
        assert!(geometry.visible_page_indexes(0.0, 600.0, 0.0).len() < 4);
    }

    #[test]
    fn current_page_has_stable_boundary_anchor() {
        let geometry = geometry();
        assert_eq!(geometry.current_page(0.0, 390.0, 0), Some(0));
        assert_eq!(geometry.current_page(0.0, 400.0, 0), Some(0));
        assert_eq!(geometry.current_page(405.0, 100.0, 1), Some(1));
    }

    #[test]
    fn jump_offsets_stay_in_native_page_domain() {
        let geometry = geometry();
        assert_eq!(geometry.jump_offset(99, 200.0, 0.1), geometry.jump_offset(3, 200.0, 0.1));
        assert!(geometry.jump_offset(2, 200.0, 0.5) > geometry.jump_offset(1, 200.0, 0.5));
    }

    #[test]
    fn mixed_aspect_pages_keep_stable_stack_geometry() {
        let geometry = PdfViewportGeometry::new(
            &[PdfPageGeometry::new(0.7), PdfPageGeometry::new(1.8)], 420.0, 12.0,
        );
        assert!(geometry.slots()[0].height > geometry.slots()[1].height);
        assert_eq!(geometry.slots()[0].width, geometry.slots()[1].width);
        assert_eq!(geometry.total_height(), geometry.slots()[1].bottom());
    }

    #[test]
    fn jump_anchor_preserves_intra_page_focus() {
        let geometry = geometry();
        let top = geometry.jump_offset(2, 600.0, 0.25);
        let centered = geometry.jump_offset(2, 600.0, 0.5);
        assert!(centered < top);
        assert!((top - centered - 150.0).abs() < f32::EPSILON);
    }
}
