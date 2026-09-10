# LanternLeaf Current Status

Updated: 2026-09-10 after Goal 0008 final real-desktop acceptance and Goal 0009 authorization.

This file contains current verified/bounded state. Detailed historical correction lineage lives in `docs/work/reports/` and `docs/work/reviews/`.

## Workspace / architecture

**VERIFIED / AUTHORITATIVE**

- Native Rust + `eframe`/`egui` is the production desktop architecture.
- Workspace includes the root package plus `lanternleaf-core`, `lanternleaf-app`, and `lanternleaf-egui`.
- Rust owns canonical document/session/playback state.
- Tauri/React/WebView is historical reference only.
- Windows human workflow is repo-native: `git pull -> .\qa.ps1`.
- Scoop is the active Windows CLI dependency convention.

## Gate 0 — Windows baseline

**COMPLETE**

Hosted Windows CI covers MSVC/Pandoc setup, workspace check/build/test, repo-native QA preparation, Windows TTS synthesis/decode, and a separate hosted renderer probe.

## Gate 1 — backend-neutral TTS + Windows TTS

**COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted runtime shape:

`canonical display sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession cursor -> native UI projection`

Verified on the human Windows machine:

- Windows speech is audible;
- Play and Pause work;
- installed Windows voices can be changed interactively;
- voice changes preserve the reader session;
- first-sample sentence boundaries now drive visible sentence synchronization instead of predicted-duration timers.

Piper exists as a backend, but Windows live-switch readiness/recovery is not yet robust. Goal 0009 owns that bounded defect; full Piper model/voice provisioning remains future work.

## Goal 0008 / Gate 2.5 — Caliberate first-class library service

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Accepted relationship:

`Caliberate 127.0.0.1:8181 -> versioned HTTP/JSON provider -> existing library browser -> materialized source -> normal ReaderSession/TTS`

Verified:

- Caliberate is the preferred local provider while legacy Calibre compatibility remains available;
- large Caliberate catalog retrieval is Arc-backed and no longer copied per frame;
- supported book formats materialize into the normal source/session path;
- valid EPUB materialization/native ingestion and cache recovery are covered;
- the previously problematic large EPUB opens quickly and native UI interaction is snappy;
- bounded pretty rendering avoids laying out the entire large document every frame;
- one canonical shared ReaderSession serves reader effects, TTS, and persistence;
- TTS/persistence hot paths avoid heavyweight full ReaderSnapshot construction;
- native EPUB structured sentence identity is source-born and carried through TTS and pretty rendering;
- semantic first-sample audio boundaries carry canonical sentence identity;
- active TTS wakes egui without depending on incidental user input;
- real-desktop pretty-view highlight now tracks the actually audible sentence accurately;
- real-desktop pretty viewport follows playback correctly;
- Windows audio, Play/Pause, and Windows voice selection work on the same real book.

Authoritative A8.3 implementation: `771c31ae90e0bf331195979fffebac8eb2f4ffd6`.

Authoritative A8.3 Windows CI: `34431615684`.

Final acceptance is recorded in `docs/work/reviews/0008-a8.3-director-acceptance.md`.

## Goal 0009 — TTS playback polish and layered voice configuration

**READY / NEXT AUTHORIZED GOAL**

Goal 0009 owns four residuals from the successful Goal 0008 QA:

- occasional unsolicited replay of a just-finished audio sentence/item while pretty highlight remains synchronized;
- text-only scroll follows correctly but its visual sentence highlight is missing/broken;
- failed/unready Piper selection can leave TTS unusable for the current session until reopen;
- configuration ownership needs explicit app-default -> per-book override layering.

Configuration target:

```text
compiled/platform defaults
        ↓
app conf/config.toml
        ↓
per-book reader overrides
        ↓
live session
```

On Windows, a new/unoverridden book should prefer **Zira** through a portable app-level voice preference. An explicit voice selected for a book should persist as that book's stable voice-ID override. A book with no override continues to inherit future app-default changes.

The current legacy per-book cache serializes a whole `AppConfig`, while the loader explicitly replaces cached `tts_backend` and `windows_voice_id` with app/base values. Goal 0009 replaces that inverse ownership with explicit optional book overrides and safe migration.

Piper scope in 0009 is recovery/readiness only: a failed Piper attempt must not poison the session, must not persist a broken backend, and Windows playback must be recoverable without reopening the book. Full Piper model downloading/catalog UX remains deferred.

## Non-PDF reader status

**CORE/PRETTY PATH STRONG; BOUNDED TEXT-ONLY POLISH REMAINS**

TXT/Markdown/HTML/EPUB automated parity remains covered from Goal 0006 onward. The large real EPUB now provides strong real-desktop evidence for native pretty rendering, scrolling, Windows speech, and canonical spoken-sentence synchronization.

Known remaining non-PDF presentation defect: text-only mode scrolls to the correct sentence during TTS but does not visibly highlight that row. Goal 0009 owns the rendering fix.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY IS NEXT AFTER GOAL 0009**

Earlier work repaired bounded PDF classification/OCR/reading-order/cache contracts. After Goal 0009 closes the residual TTS/readability polish, the next core product gate is native PDF page rendering/viewport/texture stability, followed by PDF text/TTS/highlight synchronization.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

The repository is the durable agent-to-agent communication surface. Only the single goal under `docs/work/ready/` is authorized to start. Codex owns implementation/validation/reporting and goal notifications; ChatGPT owns goal definition/review/integration; the human owns local GUI/audio verification only when requested.