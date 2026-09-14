# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

Reproducible Windows build/check/test, native-egui launch, repo-native `deps.ps1` / `qa.ps1`, Windows CI, separate renderer probe, and Scoop dependency convention are established.

## P1 — Backend-neutral TTS + Windows speech

**WORKING WINDOWS PATH COMPLETE**

Canonical reader/session semantics are backend-neutral; ordinary WinRT Windows voice enumeration/synthesis/playback works; first-sample boundaries drive canonical playback identity; physical Windows speech and interactive ordinary voice changes are verified.

## P2 — Non-PDF reader/TTS

**CURRENT TXT / MARKDOWN / HTML / EPUB PATH COMPLETE**

The accepted non-PDF path includes ingestion/parity, responsive native pretty rendering, spoken-sentence highlight/follow, ordinary Windows TTS, layered voice configuration, presentation controls, inline imagery, and stable reflow anchoring.

## P2.5 — First-class Caliberate reader integration

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Stable sustained playback, synchronized pretty/text-only highlighting, Zira app-default inheritance, per-book voice overrides, transactional Piper rejection/recovery, Close Book, and Safe Quit are accepted.

## P2.7 — Goal 0012: pretty presentation controls + inline images

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Goal 0012 is closed with accepted presentation controls, inline imagery, geometry invalidation, font fallback safety, bounded worker architecture, readable tables/TOCs, restrained blockquotes, and durable canonical TTS highlighting/follow.

## P2.8 — Goal 0013: starter shell responsive containment

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Implementation `58ae9de` uses actual center width, a deterministic `1120px` two-column breakpoint, one-column fallback below it, bounded starter groups, wrapped controls, and bounded long-content presentation while preserving Calibre virtualization and off-render-thread work.

## P2.9 — Goal 0014: reader presentation-geometry anchor stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

A3 removed the prior violent multi-frame reflow excursions while preserving canonical highlight ownership, TTS-follow precedence, bounded virtualization, and explicit user-scroll authority.

A minor residual under severe text-metric edits is split to Goal 0015 rather than keeping Goal 0014 open indefinitely.

## P2.10 — Goal 0010: Caliberate catalog covers + provider availability UX

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The provider contract and lazy-cover architecture are integrated in both repositories. Caliberate serves `/api/v1/books/{id}/cover`; LanternLeaf carries `has_cover`, bounds/coalesces visible-row work, keeps request/disk/decode work off the GUI thread, and handles per-book completion ownership safely.

Real-desktop closure verified covers before first open, continued lazy cover population while scrolling, extremely fast representative EPUB opens, and preservation of EPUB TTS / pretty-reader / visual-settings behavior. Do not resurrect the withdrawn materialization defect from the test where Caliberate itself was not running.

## P2.11 — Goal 0016: starter library live-state continuity

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The real 105,570-book Caliberate catalog now publishes progressively into usable starter state while its full walk continues off-thread, exposes truthful loaded/total progress, preserves live covers through final reconciliation, and refreshes Recents in the same process after successful source persistence.

Physical closure verified durable Recents after restart, immediate warm EPUB reopen, and preserved EPUB TTS/visual behavior. See `docs/work/reviews/0016-a2-real-desktop-acceptance.md`.

## P2.12 — Goal 0015: highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

During severe text-metric/media-related reflow, an already-visible canonical highlight can drift farther than desired before ordinary auto-follow restores it. Preserve a temporary transaction-scoped viewport band without permanent highlight pinning or fighting user scroll.

## P2.13 — Goal 0017: progressive cover backpressure + cached-hydration scaling

**QUEUED — MINOR POLISH**

Normal provider pressure during a huge cold catalog walk should not present as repeated scary cover failures or immediate retry churn. Warm startup must also stop repeatedly scanning roughly the entire cached catalog merely to rediscover already-cached thumbnails. Add bounded transient backoff/retry, lazy or indexed cached-thumbnail association, theme-aware readable errors, and bounded logging/cache rewrites.

## P2.14 — Goal 0018: Windows QA bootstrap idempotence

**QUEUED — INFRASTRUCTURE**

Repeated repo-native QA runs in one PowerShell process must not accumulate Visual Studio environment state until `VsDevCmd.bat` fails with `The input line is too long`.

## P2.15 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate, implement, or test Natural/Narrator/HD voices for now. Preserve the existing ordinary Windows voice backend and current Zira/per-book override behavior.

## P3 — Goal 0019: native PDF visual stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

LanternLeaf now has a functioning native Pdfium/egui visual PDF reader. Physical Windows QA verified a real 638-page Caliberate PDF opening through the native Reader, truthful native page-domain ownership, working Next/Previous navigation, aggressive ordinary browsing, and no representative EPUB regression.

The accepted foundation includes one authoritative `PdfNativeService` / one native Pdfium owner; visual-first open independent of Quack-check/transcript/OCR; all native/heavy PDF work off the egui thread; stale-safe source/page/zoom identity; current-priority rendering; bounded texture residency; real logical zoom; and terminal failure/panic behavior.

## P3.1 — Goal 0020: continuous native PDF viewport + practical zoom

**READY NEXT — CURRENT SUBSTANTIVE PRODUCT PRIORITY**

Replace the temporary one-page-at-a-time PDF presentation with a continuous virtualized page stack while preserving explicit native page identity/navigation and the accepted Goal-0019 architecture.

Required UX includes continuous page-boundary scrolling, partial adjacent pages, bounded visible/overscan rendering, stable current-page derivation, Next/Previous as viewport jumps, Fit width, Fit page, Reset/100%, substantially broader bounded manual zoom, stable focal anchoring during zoom/resize, and immediate-mode responsiveness on long documents.

Do not bundle PDF TTS/highlight/OCR synchronization into this goal. The authoritative contract is `docs/work/ready/0020-continuous-pdf-viewport-and-zoom.md`.

## P4 — PDF text/TTS/highlight synchronization

After Goal 0020: canonical sentence/page mapping, geometry confidence/overlays, first-sample playback identity, auto-follow/jump behavior, OCR/degraded modes, Quack-check hostile-PDF recovery integration, and representative regression corpus.

## P5 — Format expansion / ingestion hardening

DOCX/Word, further HTML edge cases, shared source/document boundaries, and broader format fixtures.

## P6 — Ergonomics, latency, packaging

Startup/TTS latency, measured cold-open performance, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and dependency cleanup justified by measured problems.
