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

Responsive two-column/one-column behavior and containment are accepted.

## P2.9 — Goal 0014: reader presentation-geometry anchor stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Violent multi-frame reflow excursions are fixed. Minor severe-reflow highlight-band polish remains Goal 0015.

## P2.10 — Goal 0010: Caliberate catalog covers + provider availability UX

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The explicit provider contract and lazy-cover path are integrated and physically verified.

## P2.11 — Goal 0016: starter library live-state continuity

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The real 105,570-book catalog publishes progressively, preserves live cover state, and refreshes/persists Recents correctly.

## P2.12 — Goal 0015: highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

Preserve a temporary viewport band for an already-visible canonical highlight during severe reflow without fighting user scroll.

## P2.13 — Goal 0017: progressive cover backpressure + cached-hydration scaling

**QUEUED — MINOR POLISH**

Stop repeated roughly-O(total-books) cached-thumbnail rediscovery and improve transient provider pressure behavior/logging.

## P2.14 — Goal 0018: Windows QA bootstrap idempotence

**QUEUED — INFRASTRUCTURE**

Repeated repo-native QA runs in one PowerShell process must not accumulate Visual Studio environment state until `VsDevCmd.bat` fails.

## P2.15 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate or test Natural/Narrator/HD voices for now.

## P3 — Goal 0019: native PDF visual stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

LanternLeaf has a functioning native Pdfium/egui visual PDF reader with one authoritative process-wide Pdfium owner and visual open independent of text/OCR recovery.

## P3.1 — Goal 0020: continuous native PDF viewport + practical zoom

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Physical QA verified continuous adjacent-page scrolling, extremely responsive rapid movement, responsive page identity, continuous-stack Previous/Next jumps, Fit Width/Fit Page/Reset, manual 25–400% zoom, high-zoom horizontal access, approximate focal preservation, and no representative EPUB/TTS regression.

## P3.2 — Goal 0021: fit-to-manual zoom transition polish

**QUEUED — MINOR POLISH**

After Fit Width/Fit Page, `+/-` should step from the current effective fit percentage rather than the remembered prior manual level.

## P4 — Goal 0022: trustworthy native PDF embedded text + TTS

**READY A7 — CURRENT SUBSTANTIVE PRODUCT PRIORITY**

A1 through A6 were rejected before human QA; none is accepted for physical testing.

A6 fixed the correctness gaps from A5: the shared trusted document now owns later-page/global canonical sentence identity, first-sample identity and final-document exhaustion are covered, current live search is reconciled off-thread with query/source/generation stale safety, and trusted publication has an explicit bounded API. Fresh hosted Windows `native-workspace` and `hosted-renderer-probe` validation passed.

A7 owns two remaining director blockers:

- replace enriched `ReaderSnapshot`/session per-drop `std::thread::spawn()` with one bounded shared off-egui retirement worker/queue so rapid continuous-page ownership, navigation, search, and settings cannot create one OS thread per retired snapshot;
- make the explicit enriched-PDF projection actually independent of total native page count by using prepared prefix indexes for global sentence bases, before/after-page checks, final-exhaustion checks, TTS display IDs, and global-sentence -> page lookup.

Hard exclusions remain: Quack-check, Python, Docling, OCR, exact sentence overlays, and hostile-PDF recovery.

The visual reader must remain usable before, during, and after text/cache/search enrichment failure.

Authoritative A7 contract: `docs/work/ready/0022-native-pdf-embedded-text-tts.md`.
A6 rejection: `docs/work/reviews/0022-a6-director-rejection.md`.
Architecture boundary: `docs/architecture/pdf-text-recovery-boundary-2026-09.md`.

## P4.1 — Native PDF sentence geometry/highlight/follow

**NEXT AFTER GOAL 0022**

For accepted embedded-text PDFs, build native sentence -> page-relative geometry, spoken overlays, continuous-viewport auto-follow/jump semantics, and explicit confidence downgrade behavior without changing canonical text ownership.

## P4.2 — Hostile/mixed PDF recovery

**AFTER NATIVE TRUSTWORTHY PATH IS PHYSICALLY ACCEPTED**

Only then harden a typed recovery-provider boundary for mixed/scanned/hostile PDFs. Quack-check/Docling/OCR may be reused or rewritten behind that boundary. They remain optional subordinate background work and can never block visual PDF open.

## P5 — Format expansion / ingestion hardening

DOCX/Word, further HTML edge cases, shared source/document boundaries, and broader format fixtures.

## P6 — Ergonomics, latency, packaging

Startup/TTS latency, measured cold-open performance, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and dependency cleanup justified by measured problems.
