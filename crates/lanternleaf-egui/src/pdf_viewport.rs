//! Deterministic, lightweight geometry for the continuous native PDF surface.
//!
//! This module deliberately knows nothing about Pdfium or textures.  It lets the
//! egui thread reserve stable page slots and select a bounded render window while
//! native raster work remains owned by the worker.

pub(crate) const PDF_BASE_PAGE_WIDTH: f32 = 816.0;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PdfViewportWitness {
    pub page_index: usize,
    pub vertical_fraction: f32,
    pub horizontal_fraction: f32,
    pub viewport_fraction: f32,
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
        let start = self.slots.partition_point(|slot| slot.bottom() < top);
        let end = self.slots.partition_point(|slot| slot.top <= bottom);
        self.slots[start.min(end)..end].iter().map(|slot| slot.index).collect()
    }

    /// The page containing the viewport anchor.  A small hysteresis band around
    /// the midpoint prevents a one-pixel boundary oscillation while scrolling.
    pub(crate) fn current_page(&self, viewport_top: f32, viewport_height: f32, previous: usize) -> Option<usize> {
        if self.slots.is_empty() { return None; }
        let anchor = viewport_top.max(0.0) + viewport_height.max(1.0) * 0.5;
        let candidate = self.slots.partition_point(|slot| slot.bottom() < anchor)
            .min(self.slots.len() - 1);
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

    pub(crate) fn capture_witness(
        &self,
        scroll_x: f32,
        scroll_y: f32,
        viewport_width: f32,
        viewport_height: f32,
        previous_page: usize,
    ) -> Option<PdfViewportWitness> {
        let page_index = self.current_page(scroll_y, viewport_height, previous_page)?;
        let slot = self.slots[page_index];
        Some(PdfViewportWitness {
            page_index,
            vertical_fraction: ((scroll_y + viewport_height * 0.5 - slot.top) / slot.height).clamp(0.0, 1.0),
            horizontal_fraction: ((scroll_x + viewport_width * 0.5) / slot.width).clamp(0.0, 1.0),
            viewport_fraction: 0.5,
        })
    }

    pub(crate) fn restore_witness(
        &self,
        witness: PdfViewportWitness,
        viewport_width: f32,
        viewport_height: f32,
    ) -> (f32, f32) {
        let slot = self.slots[witness.page_index.min(self.slots.len().saturating_sub(1))];
        (
            (slot.top + slot.height * witness.vertical_fraction - viewport_height * witness.viewport_fraction).max(0.0),
            (slot.width * witness.horizontal_fraction - viewport_width * 0.5).max(0.0),
        )
    }
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

    #[test]
    fn witness_restores_same_page_and_focal_fractions_after_resize() {
        let before = geometry();
        let witness = before.capture_witness(20.0, 450.0, 300.0, 300.0, 1).unwrap();
        let after = PdfViewportGeometry::new(&[PdfPageGeometry::new(1.0); 4], 600.0, 16.0);
        let (y, x) = after.restore_witness(witness, 400.0, 500.0);
        let restored = after.capture_witness(x, y, 400.0, 500.0, witness.page_index).unwrap();
        assert_eq!(restored.page_index, witness.page_index);
        assert!((restored.vertical_fraction - witness.vertical_fraction).abs() < 0.02);
    }

    #[test]
    fn long_document_lookup_returns_a_small_window() {
        let pages = vec![PdfPageGeometry::new(0.707); 100_000];
        let geometry = PdfViewportGeometry::new(&pages, 800.0, 16.0);
        let visible = geometry.visible_page_indexes(50_000.0, 50_800.0, 900.0);
        assert!(visible.len() < 10);
        let current = geometry.current_page(50_000.0, 800.0, 10).unwrap();
        assert!(current < 100_000);
        assert_eq!(geometry.current_page(50_000.0, 800.0, current), Some(current));
    }
}
