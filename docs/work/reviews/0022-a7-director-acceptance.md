# Goal 0022 A7 — director acceptance before real-desktop QA

## Decision

**ACCEPTED FOR REAL-DESKTOP QA.** Goal 0022 remains open until physical Windows behavior is verified.

A7 closes the two A6 director blockers without weakening the accepted Goal 0019/0020 visual architecture.

## Accepted source/architecture evidence

- The per-snapshot `std::thread::spawn()` retirement path is gone. Enriched `ReaderSnapshot`, `ReaderSession`, and trusted-document replacement hand document-scale ownership to one process-wide `lanternleaf-pdf-retirement` worker through a nonblocking sender.
- Rapid snapshot churn therefore uses one stable retirement worker rather than creating an OS thread per page/snapshot replacement.
- The enriched projection remains explicitly published through `snapshot_enriched_pdf_bounded()`.
- Enriched global canonical identity now uses the prepared document's prefix indexes rather than legacy pre-enrichment pagination.
- `global_display_idx()`, TTS page bases, before/after-page checks, stats, bookmarks, and canonical page lookup use prepared prefix state; global sentence -> native page lookup is binary-search based.
- A6's stale-safe live-search reconciliation, correct later-page canonical identity, first-sample identity, true final-document exhaustion, shared immutable trusted-document ownership, and exact-visual-sync-disabled policy are preserved.
- Quack-check, Python, Docling, OCR, and hostile-PDF recovery remain excluded from this baseline trusted embedded-text path.

## Deterministic/hosted evidence

Substantive A7 commit: `9dc06b630eb35b19e725f79d012144450e196957`.

Fresh hosted Windows workflow `34973136075` completed successfully on that commit.

- `native-workspace` job `104394075799`: check/build/workspace tests, QA preparation, ordinary Windows TTS diagnostics, and notification gates passed.
- `hosted-renderer-probe` job `104397538929`: shared Pdfium metadata/raster lifecycle, cooperative native text arbitration, Nearby raster arbitration, and renderer capability probes passed.
- `enriched_pdf_snapshot_churn_uses_one_bounded_retirement_worker` exercises 256 enriched snapshot replacements plus session retirement and observes one off-caller retirement worker identity.
- `enriched_pdf_bounded_projection_uses_prefix_indexes_for_small_and_large_documents` exercises 10-page and 100-page enriched fixtures and asserts no enriched linear-scan fallback in the bounded projection helpers.

The final branch was squash-integrated through PR #29 as main commit `f19728157fd683b62c95ec40b72a2bf719202b9f`.

## Physical QA now authorized

Use a real text-bearing PDF through the normal Caliberate/native reader path. Verify:

1. Visual PDF open remains immediate/usable and continuous scrolling remains as responsive as Goal 0020.
2. After embedded-text enrichment lands, Text-only becomes available and contains coherent page-aligned text.
3. Ordinary Windows TTS becomes available and audibly advances across multiple sentences and at least one native page boundary without replaying or restarting from an earlier sentence.
4. Pause/Play, Next/Previous sentence, and ordinary installed voice behavior remain functional.
5. Search for a phrase on the current page and a phrase known to exist on a later page. Search Next/Previous must navigate to the correct native page/text sentence without requiring the query to be retyped after enrichment.
6. Scroll rapidly across many pages after enrichment and confirm the visual reader remains smooth/responsive; no progressive stalls or thread/churn symptoms should appear.
7. Switch between visual PDF and Text-only, then return to visual mode; native page identity should remain coherent.
8. Close/reopen the same PDF and confirm the warm trusted-text path is materially quicker/does not visibly re-block visual open.
9. Open a representative EPUB afterward and verify visual reading + ordinary TTS still work.

Do not test Natural/HD voices. Exact sentence rectangles/highlight overlays are not part of Goal 0022 and should remain absent/disabled.

## Acceptance consequence

If physical QA passes, close Goal 0022 as the trustworthy native embedded-text/TTS baseline and advance Gate 4 to native PDF sentence geometry/highlight/follow. Hostile/mixed/scanned recovery remains a later separate gate behind the accepted native baseline.
