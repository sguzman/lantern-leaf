# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

Reproducible Windows build/check/test, native egui launch, repo-native `deps.ps1` / `qa.ps1`, Windows CI, separate renderer probe, and Scoop dependency convention are established.

## P1 — Backend-neutral TTS + Windows speech

**WORKING WINDOWS PATH COMPLETE**

Canonical reader/session semantics are backend-neutral; Windows voice enumeration/synthesis/playback works; first-sample boundaries drive canonical playback identity; physical Windows speech and interactive voice changes are verified.

Full Piper model/catalog/downloader UX remains future work.

## P2 — Non-PDF reader/TTS

**PRETTY EPUB PATH ACCEPTED; GOAL 0009 A1 DESKTOP SIGNOFF PENDING**

- TXT/Markdown/HTML/EPUB automated parity is established;
- native pretty rendering is bounded and responsive on the real large EPUB;
- pretty spoken-sentence highlight and viewport follow are accurate on the real Windows machine;
- Goal 0009 A1 now makes text-only row selection consume the same high-frequency canonical playback ID as follow/scroll; visible real-desktop confirmation remains.

## P2.5 — First-class Caliberate service

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**A1 ACCEPTED — HUMAN SIGNOFF ONLY**

Accepted implementation: `52ae85dad02f2e5588c14d33817abf0c5db69916`.

Windows CI: `34517286850`.

Implemented:

- portable app-level Windows voice preference, checked in as Zira;
- explicit versioned per-book overrides rather than frozen whole-AppConfig persistence;
- safe legacy book-config migration;
- explicit per-book voice/backend persistence semantics;
- continuation-cursor repair for TTS refill/window progression, with 300 ordered deterministic boundaries and no ordinary duplicate starts;
- canonical text-only row selection/highlight ownership;
- transactional Piper readiness validation that rejects an unusable switch before canonical session mutation.

One repo-native real-desktop run must confirm: no unsolicited replay over a sustained stretch, visible text-only highlighting, actual Zira inheritance, per-book voice reopen persistence, and same-session Windows playback after an unavailable Piper attempt.

Do not authorize PDF implementation until this signoff is known.

## P3 — Native PDF visual stability

**NEXT CORE PRODUCT GATE AFTER GOAL 0009 SIGNOFF**

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

Startup/TTS latency, UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and measured dependency cleanup.
