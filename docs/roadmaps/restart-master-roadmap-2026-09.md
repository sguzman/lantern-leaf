# LanternLeaf Restart Master Roadmap — September 2026

This is the active restart roadmap for the Windows/native-egui line. Completion is evidence-driven: accepted implementation plus the human-only runtime evidence a gate actually requires.

## Gate 0 — Trustworthy Windows baseline

**STATUS: COMPLETE**

Goals 0001–0005 established reproducible Windows CI, deterministic cache/test behavior, native-egui launch, and repaired bounded core contracts.

## Gate 1 — Backend-neutral TTS + Windows TTS

**STATUS: COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted flow:

`canonical sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession -> native UI`

Windows speaker playback, Play/Pause, and installed Windows voice switching are now verified on the real Windows machine.

Piper remains a supported backend but Windows provisioning/live-switch recovery is not yet polished. Goal 0009 owns bounded recovery; a complete Piper model catalog/downloader is future work.

## Workflow UX — macro-goal notifications

**STATUS: IMPLEMENTED / MULTI-ATTEMPT HARDENED**

Repository goal identity is durable; Codex Goal sessions are disposable attempts. Correction attempts reuse the repository goal ID, re-arm the watcher, push before signaling terminal state, and return the shared checkout to `main`.

## Gate 2 — Non-PDF reader/TTS

**STATUS: CORE/PRETTY EPUB PATH ACCEPTED; TEXT-ONLY VISUAL POLISH IN GOAL 0009**

Goal 0006 established automated parity for TXT, Markdown, HTML, and EPUB. Goal 0008's later synchronization work strengthened native EPUB identity and real-desktop behavior substantially.

Real Windows evidence now includes:

- fast/snappy large EPUB opening and interaction;
- working Windows speech;
- working Play/Pause and Windows voice changes;
- accurate native pretty spoken-sentence highlighting;
- accurate viewport follow without the prior random jumps;
- text-only viewport follow using the same canonical playback identity.

Known residual: text-only row highlighting is visually broken even though its scroll follows correctly. Goal 0009 owns this bounded rendering defect.

Human workflow remains:

`git pull -> .\qa.ps1`

No ordinary manual QA uses downloaded CI artifacts.

## Gate 2.5 — First-class Caliberate library service

**STATUS: COMPLETE — GOAL 0008 CLOSED**

Accepted relationship:

`Caliberate -> HTTP/JSON v1 at 127.0.0.1:8181 -> existing library browser -> materialized source -> normal reader/TTS pipeline`

Goal 0008 now has both automated and real-desktop acceptance evidence:

- paged large Caliberate catalog works through the existing UI;
- supported books materialize into the normal source/session path;
- legacy Calibre compatibility remains;
- large real Caliberate EPUB opens quickly and remains responsive;
- Windows speech works on the materialized book;
- canonical first-sample playback boundaries, pretty highlight, and viewport follow stay synchronized on the real machine.

The long A3–A8.3 correction history is preserved under `docs/work/reports/0008.md` and `docs/work/reviews/`.

## Gate 2.6 — TTS playback polish + layered voice configuration

**STATUS: READY — GOAL 0009**

Before PDF work, close the residual reader/TTS defects discovered during Goal 0008 acceptance:

- occasional unsolicited replay of a just-finished audio item;
- missing text-only visual highlight despite correct scroll-follow;
- failed/unready Piper selection poisoning the current TTS session;
- app-default/per-book voice/backend ownership.

Configuration policy:

`compiled/platform defaults -> app conf/config.toml -> per-book overrides -> live session`

New/unoverridden Windows books should portably prefer Zira. Explicit per-book voice changes persist as stable voice-ID overrides. Full Piper voice/model management is outside this gate.

## Gate 3 — Native PDF visual stability

**STATUS: NEXT CORE RENDERING GATE AFTER GOAL 0009**

Focus:

- page raster/rendering;
- texture/cache lifecycle;
- viewport scheduling;
- zoom/scroll stability;
- bounded memory/performance;
- reliable visual behavior independent of TTS.

## Gate 4 — PDF text, TTS, and highlight synchronization

After Gate 3:

- canonical sentence/page mapping;
- geometry confidence;
- overlays;
- first-sample audio-boundary identity;
- jump/follow behavior;
- OCR/degraded modes;
- representative regression corpus.

## Gate 5 — Format expansion and ingestion cleanup

- DOCX/Word;
- further HTML edge cases;
- common source/document boundaries;
- broader format fixtures.

## Gate 6 — Ergonomics, performance, packaging

- startup/TTS latency;
- UI cleanup;
- large-document ergonomics;
- Piper model/voice management if desired;
- library/import polish;
- release packaging;
- dependency cleanup justified by measured problems.

## Director rule

ChatGPT may combine related repair passes into one macro-goal when doing so removes needless human/Codex round trips without opening architectural ambiguity. Current verified state and active goal contracts outrank historical roadmap text.