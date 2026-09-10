# LanternLeaf Restart Master Roadmap — September 2026

This is the active restart roadmap for the Windows/native-egui line. Completion is evidence-driven: accepted implementation plus the human-only runtime evidence a gate actually requires.

## Gate 0 — Trustworthy Windows baseline

**STATUS: COMPLETE**

Goals 0001–0005 established reproducible Windows CI, deterministic cache/test behavior, native-egui launch, and repaired bounded core contracts.

## Gate 1 — Backend-neutral TTS + Windows TTS

**STATUS: COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted flow:

`canonical sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession -> native UI`

Windows speaker playback, Play/Pause, and installed Windows voice switching are verified on the real Windows machine.

## Workflow UX — macro-goal notifications

**STATUS: IMPLEMENTED / MULTI-ATTEMPT HARDENED**

Repository goal identity is durable; Codex Goal sessions are disposable attempts. Correction attempts reuse the repository goal ID, re-arm the watcher, push before signaling terminal state, and return the shared checkout to `main`.

## Gate 2 — Non-PDF reader/TTS

**STATUS: CORE/PRETTY PATH ACCEPTED; GOAL 0009 A1 DESKTOP POLISH SIGNOFF PENDING**

Goal 0006 established automated parity for TXT, Markdown, HTML, and EPUB. Goal 0008 strengthened native EPUB identity and proved fast responsive pretty rendering, audible Windows speech, accurate spoken-sentence highlight, and viewport follow on the real machine.

Goal 0009 A1 is now integrated for human verification. Text-only row selection is driven by the same high-frequency canonical playback ID as follow/scroll, but real visible styling still requires desktop confirmation.

Human workflow remains:

`git pull -> .\qa.ps1`

No ordinary manual QA uses downloaded CI artifacts.

## Gate 2.5 — First-class Caliberate library service

**STATUS: COMPLETE — GOAL 0008 CLOSED**

Accepted relationship:

`Caliberate -> HTTP/JSON v1 at 127.0.0.1:8181 -> existing library browser -> materialized source -> normal reader/TTS pipeline`

Large-catalog behavior, materialization, native EPUB ingestion, responsive rendering, Windows speech, canonical first-sample boundaries, pretty highlighting, and viewport follow have automated plus real-desktop acceptance.

## Gate 2.6 — TTS playback polish + layered voice configuration

**STATUS: A1 ACCEPTED FOR REAL-DESKTOP QA**

Goal 0009 A1 implements:

- portable app-level Windows voice preference with checked-in Zira default;
- explicit versioned per-book overrides and legacy migration;
- app-default inheritance for unoverridden books and explicit book-local voice/backend intent;
- TTS continuation state across eight-item refill batches and bounded 64-sentence plan windows;
- a 300-boundary deterministic ordinary-playback regression with exact ordered starts;
- text-only row selection from the live canonical playback identity;
- transactional validation of backend/voice changes so unavailable Piper resources are rejected before session mutation.

Authoritative implementation: `52ae85dad02f2e5588c14d33817abf0c5db69916`.

Authoritative Windows CI: `34517286850`.

Exit requires one real-Windows run confirming no unsolicited replay over sustained playback, visible text-only highlight, Zira inheritance, book-level voice persistence, and immediate Windows recovery after an unavailable Piper attempt without reopening the book.

Full Piper voice/model management remains outside this gate.

## Gate 3 — Native PDF visual stability

**STATUS: NEXT, NOT YET AUTHORIZED**

After Gate 2.6 human signoff:

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

Startup/TTS latency, UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and dependency cleanup justified by measured problems.

## Director rule

ChatGPT may combine related repair passes into one macro-goal when doing so removes needless human/Codex round trips without opening architectural ambiguity. Current verified state and active goal contracts outrank historical roadmap text.
