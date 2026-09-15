# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

Reproducible Windows build/check/test, native-egui launch, repo-native `deps.ps1` / `qa.ps1`, Windows CI, separate renderer probe, and Scoop dependency convention are established.

## P1 — Backend-neutral TTS + Windows speech

**WORKING WINDOWS PATH COMPLETE**

Canonical reader/session semantics are backend-neutral; ordinary Windows speech works; first-sample boundaries drive canonical playback identity; physical speech and ordinary installed voice switching are verified.

## P2 — Non-PDF reader/TTS

**ACCEPTED BASELINE; GOAL 0022 CURRENTLY OWNS ONE STALE-EPUB REOPEN REGRESSION**

EPUB/TXT/Markdown/HTML native Pretty/Text-only/TTS/highlight/follow are broadly accepted. A8/A9 fixed the active relative Text-only race, but A10 must prove a previously damaged EPUB cannot reopen with stale truncated presentation state.

## P2.5 — First-class Caliberate reader integration

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Stable sustained playback, synchronized non-PDF Pretty/Text-only highlighting, Zira inheritance, per-book voice overrides, transactional Piper rejection/recovery, Close Book, and Safe Quit are accepted.

## P2.7 — Goal 0012: pretty presentation controls + inline images

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Presentation controls, inline imagery, geometry invalidation, font fallback safety, bounded worker architecture, readable tables/TOCs/blockquotes, and durable canonical TTS highlighting/follow are accepted.

## P2.8 — Goal 0013: starter shell responsive containment

**COMPLETE**

Responsive two-column/one-column behavior and containment are accepted.

## P2.9 — Goal 0014: reader presentation-geometry anchor stability

**COMPLETE**

Violent multi-frame reflow excursions are fixed. Minor severe-reflow highlight-band polish remains Goal 0015.

## P2.10 — Goal 0010: Caliberate catalog covers + provider availability UX

**COMPLETE**

The explicit provider cover contract and lazy-cover path are integrated and physically verified.

## P2.11 — Goal 0016: starter library live-state continuity

**COMPLETE**

The real ~105,570-book catalog publishes progressively, preserves live cover state, and refreshes/persists Recents correctly.

## P2.12 — Goal 0015: highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

Preserve a temporary viewport band for an already-visible canonical highlight during severe reflow without fighting user scroll.

## P2.13 — Goal 0017: progressive cover backpressure + cached hydration

**QUEUED — MINOR/SCALING POLISH**

Stop repeated roughly-O(total-books) cached-thumbnail rediscovery and improve transient provider pressure behavior/logging.

## P2.14 — Goal 0018: Windows QA bootstrap idempotence

**QUEUED — INFRASTRUCTURE**

Repeated repo-native QA runs in one PowerShell process must not accumulate Visual Studio environment state until `VsDevCmd.bat` fails.

## P2.15 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate or test Natural/Narrator/HD voices for now.

## P2.16 — Goal 0024: Caliberate materialized-source identity in Recents

**QUEUED — REAL-DESKTOP UX DEFECT**

Provider-materialized cached PDFs currently appear in Recents under hash-like filenames such as `725-13e8b7a0` and can lose the known catalog cover/title. Preserve durable Caliberate provenance/title/cover without rematerialization or O(total-catalog) lookup.

## P3 — Goal 0019: native PDF visual stability

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

LanternLeaf has a functioning native Pdfium/egui visual PDF reader with one authoritative process-wide Pdfium owner and visual open independent of text/OCR recovery.

## P3.1 — Goal 0020: continuous native PDF viewport + practical zoom

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Continuous adjacent-page scrolling, extremely responsive rapid movement, responsive page identity, continuous-stack Previous/Next jumps, Fit Width/Fit Page/Reset, manual 25–400% zoom, high-zoom horizontal access, and approximate focal preservation are physically accepted.

A9 physical QA re-confirmed violent scrollbar dragging remains fast. Transient `Rendering page N` placeholders remain brief/non-blocking.

## P3.2 — Goal 0021: fit-to-manual zoom transition polish

**QUEUED — MINOR PDF UX POLISH**

After Fit Width/Fit Page, `+/-` should step from the current effective fit percentage rather than the remembered prior manual level.

## P4 — Goal 0022: trustworthy native PDF embedded text + TTS

**READY A10 — CURRENT SUBSTANTIVE PRODUCT PRIORITY**

A1–A6 were rejected before physical QA. A7 was the first physical attempt and preserved Goal 0020 while exposing page-continuation, Search, and Text-only race defects. A8/A9 corrected major architecture/state issues and passed hosted validation.

A9 physically proved:

- native PDF visual responsiveness remains excellent;
- Text-only works/page-aligns;
- natural TTS can cross native page boundaries;
- PDF Text-only stress works;
- a newly opened EPUB survives the original stress-toggle failure.

A9 is still physically rejected because:

- TTS Speed/Volume sliders are rebuilt from asynchronous snapshots and can visibly snap back (speed observed returning to `2.5`), while emitting settings traffic continuously during drag;
- command/status diagnostics participate in unacceptable reader layout movement;
- Search accepts text and computes match counts but has no usable Previous/Next/Enter navigation or selected-match provenance/excerpt;
- burst Previous can repeat instead of moving monotonically backward;
- visual PDF has no discoverable coarse page-level TTS positioning action;
- the EPUB damaged under A7 can reopen in a stale truncated-looking state and requires diagnosis of the actual persistence/cache/session owner.

A10 owns exactly those corrections while preserving all accepted A9 native/Pdfium/text/TTS architecture.

Hard exclusions remain: Quack-check, Python, Docling, OCR, exact PDF visual sentence overlays, visual auto-follow, and hostile-PDF recovery.

Authoritative A10 contract: `docs/work/ready/0022-native-pdf-embedded-text-tts.md`.
A9 physical rejection: `docs/work/reviews/0022-a9-real-desktop-rejection.md`.

## P4.1 — Native PDF sentence geometry/highlight/follow

**NEXT ONLY AFTER GOAL 0022 PHYSICAL ACCEPTANCE**

For accepted embedded-text PDFs, build native sentence -> page-relative geometry, spoken overlays, continuous-viewport auto-follow/jump semantics, and truthful click-to-sentence behavior where confidence permits.

## P4.2 — Hostile/mixed PDF recovery

**AFTER NATIVE TRUSTWORTHY PATH IS PHYSICALLY ACCEPTED**

Only then harden a typed recovery-provider boundary for mixed/scanned/hostile PDFs. Quack-check/Docling/OCR may be reused or rewritten behind that boundary and can never block visual PDF open.

## P5 — Format expansion / ingestion hardening

DOCX/Word, further HTML edge cases, shared source/document boundaries, and broader format fixtures.

## P6 — Ergonomics, latency, packaging

Startup/TTS latency, measured cold-open performance, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and dependency cleanup justified by measured problems.
