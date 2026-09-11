# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

Reproducible Windows build/check/test, native egui launch, repo-native `deps.ps1` / `qa.ps1`, Windows CI, separate renderer probe, and Scoop dependency convention are established.

## P1 — Backend-neutral TTS + Windows speech

**WORKING WINDOWS PATH COMPLETE**

Canonical reader/session semantics are backend-neutral; ordinary WinRT Windows voice enumeration/synthesis/playback works; first-sample boundaries drive canonical playback identity; physical Windows speech and interactive voice changes are verified.

## P2 — Non-PDF reader/TTS

**PRETTY EPUB ACCEPTED; GOAL 0009 A3 TEXT-ONLY SOURCE OWNERSHIP OPEN**

- TXT/Markdown/HTML/EPUB ingestion/parity remains covered;
- native pretty rendering is bounded, responsive, and sentence-synchronized on the real large EPUB;
- ordinary Windows TTS no longer shows the prior duplicate-line refill bug in desktop QA;
- source-dependent text-only failure remains: one real EPUB has no text/highlight while another works;
- director code audit identifies text-only presentation incorrectly depending on transient `audio_sentences` / `highlighted_audio_idx`.

## P2.5 — First-class Caliberate reader integration

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**A3 REOPENED — CANONICAL TEXT-ONLY DOCUMENT PROJECTION**

A3 must:

1. derive visible text-only rows from canonical/display document state, never the TTS audio plan;
2. preserve exactly one row per canonical/display sentence even when normalization produces zero/one/many audio items;
3. use canonical spoken identity for selection/follow, never `highlighted_audio_idx`;
4. remain complete while TTS is idle/stopped/paused, has no plan, has an evicted/stale plan, or backend validation fails;
5. avoid constructing a TTS plan merely to render/snapshot text-only content;
6. add source-shaped regressions that reproduce the difference between simple and awkward EPUB streams;
7. preserve pretty sync, 300+ no-repeat behavior, Zira inheritance, per-book voice overrides, failed-Piper recovery, bounded diagnostics, Close book, Safe Quit, and stale-source isolation.

No human QA until A3 passes director review.

## P2.7 — Goal 0010: Caliberate catalog reliability

**QUEUED — DO NOT START BEFORE GOAL 0009 CLOSES**

- preserve full provider/stage/format/root-cause diagnostics when materialization fails;
- deterministic fallback only among explicitly advertised supported formats;
- recover cleanly from SourceError;
- add a first-class Caliberate cover contract if the server lacks one rather than probing invented legacy routes;
- load covers lazily for visible rows, off the GUI thread, with bounded concurrency/cache;
- never download/materialize all ~100k books or full EPUBs just for thumbnails;
- make catalog covers available before a book has been opened while preserving Recents/local cover fallback.

## P2.8 — Goal 0011: Windows Natural/HD voice capability

**QUEUED**

- research/probe supported Windows APIs first;
- distinguish ordinary WinRT `SpeechSynthesizer::AllVoices()` voices from user-installed Natural/Narrator/HD voices;
- integrate Natural/HD voices only through a supported, maintainable application API if available;
- otherwise expose the capability limitation clearly while preserving ordinary Windows TTS;
- do not use undocumented Narrator model/key extraction or brittle reverse-engineered hacks as the default path;
- preserve app-level portable voice preference and per-book override semantics;
- do not change the user's default voice until requested.

## P3 — Native PDF visual stability

**FUTURE CORE PRODUCT GATE; NOT AUTHORIZED WHILE GOAL 0009 A3 IS OPEN**

- page raster/render ownership;
- texture/cache lifecycle;
- viewport scheduling;
- zoom/scroll stability;
- bounded memory/performance on representative PDFs;
- visual behavior independent of TTS.

## P4 — PDF text/TTS/highlight synchronization

After P3: canonical sentence/page mapping, geometry confidence/overlays, jump/follow behavior, OCR/degraded modes, first-sample playback integration, and regression corpus.

## P5 — Format expansion / ingestion hardening

DOCX/Word, further HTML edge cases, shared source/document boundaries, and broader format fixtures.

## P6 — Ergonomics, latency, packaging

Startup/TTS latency, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and measured dependency cleanup.
