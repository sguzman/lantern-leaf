# LanternLeaf Restart Master Roadmap — September 2026

This is the active restart roadmap for the Windows/native-egui line. Completion is evidence-driven: accepted implementation plus the human-only runtime evidence a gate actually requires.

## Gate 0 — Trustworthy Windows baseline

**STATUS: COMPLETE**

Goals 0001–0005 established reproducible Windows CI, deterministic cache/test behavior, native-egui launch, repository-owned Windows QA, macro-goal notifications, and repaired bounded core contracts.

## Gate 1 — Backend-neutral TTS + Windows TTS

**STATUS: COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted flow:

`canonical sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession -> native UI`

Windows speaker playback, Play/Pause, ordinary installed voice switching, and first-sample-driven canonical cursor ownership are verified on the real Windows machine.

## Workflow UX — macro-goal notifications

**STATUS: IMPLEMENTED / MULTI-ATTEMPT HARDENED**

Repository goal identity is durable; Codex Goal sessions are disposable attempts. Correction attempts reuse the repository goal ID, re-arm the watcher, push before signaling terminal state, and return the shared checkout to `main`.

## Gate 2 — Non-PDF reader/TTS

**STATUS: COMPLETE FOR CURRENT TXT / MARKDOWN / HTML / EPUB READER PATH**

Goal 0006 established automated parity for TXT, Markdown, HTML, and EPUB. Goal 0008 strengthened native EPUB identity and proved fast responsive pretty rendering, audible Windows speech, accurate spoken-sentence highlight, and viewport follow on the real machine.

Goal 0009 closed sustained playback correctness, pretty/text-only synchronization, layered Windows voice configuration, Piper failure recovery, Close book, and Safe Quit behavior with automated plus real-desktop acceptance.

Human workflow remains `git pull -> .\qa.ps1`; no ordinary manual QA uses downloaded CI artifacts.

## Gate 2.5 — First-class Caliberate library service

**STATUS: COMPLETE — GOAL 0008 CLOSED**

Accepted relationship:

`Caliberate -> HTTP/JSON v1 at 127.0.0.1:8181 -> existing library browser -> materialized source -> normal reader/TTS pipeline`

Large-catalog behavior, materialization, native EPUB ingestion, responsive rendering, Windows speech, canonical first-sample boundaries, pretty highlighting, and viewport follow have automated plus real-desktop acceptance.

## Gate 2.6 — TTS playback polish + layered voice configuration

**STATUS: COMPLETE — GOAL 0009 CLOSED**

Accepted behavior includes sustained ordinary Windows playback without prior duplicate-line refill, synchronized pretty/text-only highlighting and follow, Zira app-default inheritance, per-book voice overrides, transactional Piper rejection/recovery, bounded diagnostics, Close book, and ordered Safe Quit.

## Gate 2.7 — Pretty presentation controls + inline images

**STATUS: COMPLETE — GOAL 0012 CLOSED**

Goal 0012 now has automated, director, Windows CI, and real-desktop acceptance for the native pretty-reader presentation surface.

Accepted behavior includes literal horizontal/vertical margins, geometry invalidation, scrollable settings, functional word/letter spacing, readable tables/TOCs, restrained blockquotes, explicit font fallback state, inline EPUB imagery, bounded off-render-thread pretty/image work, and durable spoken-sentence highlight/follow across geometry changes.

A final human pass confirmed the visual corrections and TTS highlight behavior. One separate residual observation remains queued as Goal 0014: changing media max-width/max-height controls can disturb viewport anchoring and prevented physical confirmation of the visible media-size effect. That does not reopen Goal 0012.

## Near-term shell / library cleanup

### Goal 0013 — starter shell responsive panel containment

**STATUS: READY — NEXT AUTHORIZED MACRO-GOAL**

Repair the still-visible Recents / Calibre / Browser Tabs overlap using explicit responsive containment, wrapped/stacked child rows, and a stable one-column fallback when two readable columns do not fit.

### Goal 0010 — Caliberate catalog covers + provider availability UX

**STATUS: QUEUED**

Add first-class lazy catalog covers, bounded visible-row loading, intentional loading/no-cover/error states, and correct provider-unavailable classification. Do not resurrect the withdrawn fake materialization defect from the test where Caliberate itself was not running.

### Goal 0014 — media-sizing anchor stability

**STATUS: QUEUED**

Make live max-width/max-height media changes preserve viewport/media anchoring and visibly affect media when the selected limit is actually binding, while preserving Goal 0012 geometry/TTS behavior.

### Goal 0011 — Windows Natural/HD voices

**STATUS: DEFERRED BY USER**

Do not investigate or test until explicitly re-authorized. Preserve the ordinary working Windows voice backend.

## Gate 3 — Native PDF visual stability

**STATUS: FUTURE CORE PRODUCT GATE; NOT CURRENTLY AUTHORIZED**

After the current near-term native shell/library cleanup sequence:

- page raster/rendering;
- texture/cache lifecycle;
- viewport scheduling;
- zoom/scroll stability;
- bounded memory/performance;
- reliable visual behavior independent of TTS.

## Gate 4 — PDF text, TTS, and highlight synchronization

After Gate 3: canonical sentence/page mapping, geometry confidence, overlays, first-sample audio-boundary identity, jump/follow behavior, OCR/degraded modes, and representative regression corpus.

## Gate 5 — Format expansion and ingestion cleanup

DOCX/Word, further HTML edge cases, common source/document boundaries, and broader format fixtures.

## Gate 6 — Ergonomics, performance, packaging

Startup/TTS latency, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and dependency cleanup justified by measured problems.

## Director rule

ChatGPT may combine related repair passes into one macro-goal when doing so removes needless human/Codex round trips without opening architectural ambiguity. Current verified state and active goal contracts outrank historical roadmap text.
