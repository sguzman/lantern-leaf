# Goal 0022 A9 — director acceptance before real-desktop QA

## Decision

**SOURCE/CI ACCEPTED — REAL-DESKTOP QA AUTHORIZED. Goal 0022 remains open until physical Windows acceptance.**

A9 closes the A8 director blockers without changing the native-first Gate-4 boundary.

Integrated to `main` through PR #30 as squash commit `fb416b02fe09a28374fb085d71a2d4a74d235ec2`.

Substantive A9 implementation commit: `8a991a3f7ce72c42e5e0138d09bcbc8f2228f66e`.

Fresh hosted Windows workflow: `34991458105`.

- `native-workspace` job `104456885795`: **passed**.
- `hosted-renderer-probe` job `104460108538`: **passed**.

## Accepted corrections

### Search input ownership

Search panel visibility remains persistent and separate from one-shot focus acquisition. A9 adds persistent app-owned knowledge of whether the real Search `TextEdit` has focus and projects that into `ShellState`, so reader shortcuts are suppressed while the editor owns keyboard input rather than merely while the initial focus request is pending.

### Search draft ownership

The Search editor no longer reconstructs its visible text from asynchronously acknowledged ReaderSession state every frame. A persistent synchronous draft buffer owns immediate typing, with deliberate source/authoritative-query reconciliation and stale-ack protection. Full-document search work remains off the egui thread.

### Runtime-level PDF page continuation

A9 adds a simulated `TtsRuntime` regression that exercises the actual runtime loop from a one-sentence title page through empty native pages and multiple later native pages. It proves monotonic first-sample canonical identity, no replay, no empty-batch failure, and one final completion.

### Global/local TTS identity boundary

`ReaderSession::apply_tts_sentence_boundary()` now uses document-global canonical identity for ownership and page-local identity for page-local normalization-plan bounds. Later pages with large global sentence bases no longer spuriously invalidate a valid local bounded plan on each first-sample event.

## Preserved architecture

- Rust + `eframe`/`egui` remains authoritative.
- Exactly one process-wide Pdfium owner remains authoritative for native PDF metadata/raster/text work.
- Visual PDF open remains independent of text/TTS/search/recovery.
- Current/Nearby raster work retains priority over background native-text extraction.
- Trusted native embedded text remains page-aligned and prefix-indexed.
- Document-scale preparation/cache/search/destruction remains off egui.
- A8 idempotent `SetTextOnly { enabled }` desired-state semantics remain.
- Exact PDF sentence rectangles, pretty-surface spoken highlight, and PDF visual auto-follow remain **out of scope** for Goal 0022.
- Quack-check, Python, Docling, OCR, scripts/quack-check, and hostile/mixed recovery remain **out of the Goal 0022 runtime path**.

## Real-desktop QA focus

Use a fresh PowerShell because Goal 0018 remains queued. Do not use `-ResetQaState` unless deliberately testing cold state.

Physical QA should concentrate on the exact failures that escaped A7 plus regression safety:

1. Open the known large text-bearing PDF and confirm continuous visual scrolling remains highly responsive under violent wheel/scrollbar movement.
2. Confirm Text-only becomes usable and page-aligned after trusted native text is available.
3. Start ordinary Windows TTS on a sparse/title/front-matter page and let it naturally cross native page boundaries without stopping, replaying, or requiring manual Next.
4. Let playback cross several page boundaries, including pages with little/no text if available.
5. Exercise Next/Previous sentence around page boundaries; no repeated same sentence solely because the page-local plan ended.
6. Open Search, type rapidly including ordinary shortcut-looking characters such as `f`, `s`, `r`, and space. The query must remain visible/ordered and typing must not trigger reader shortcuts.
7. Clear Search; the panel must remain open. Search a distinctive later-page phrase and exercise Search Next/Previous.
8. Rapidly alternate PDF visual/Text-only and confirm final requested state wins.
9. Open a representative EPUB and aggressively alternate Pretty/Text-only. Returning to Pretty must render the complete document, not a truncated fragment; ordinary EPUB TTS must remain healthy.
10. Close/reopen the PDF once and confirm warm behavior remains snappy.

Do **not** fail Goal 0022 for absence of PDF visual spoken-sentence rectangles/highlight/follow; that belongs to the next geometry/highlight/follow goal after this baseline is physically accepted.

## Director conclusion

No remaining source-level blocker found in A9. The next authority is physical Windows behavior.
