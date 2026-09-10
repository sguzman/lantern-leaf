# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

- reproducible Windows build/check/test;
- native egui application launches;
- repo-native `deps.ps1` / `qa.ps1` workflow;
- meaningful Windows CI and separate hosted renderer probe;
- Scoop remains the Windows CLI dependency convention.

## P1 — Backend-neutral TTS + Windows speech

**WORKING WINDOWS PATH COMPLETE; PIPER POLISH DEFERRED/BOUNDED**

- canonical reader/session semantics are backend-neutral;
- Windows voice enumeration/synthesis/playback works;
- first-sample audio boundaries drive canonical playback identity;
- Windows speaker playback and interactive voice changes are verified on the human machine.

Piper exists, but Windows readiness/live-switch recovery is not yet robust. Goal 0009 handles safe failure/recovery only; full Piper model/voice management is later work.

## P2 — Non-PDF reader/TTS

**PRETTY EPUB PATH ACCEPTED; TEXT-ONLY VISUAL POLISH REMAINS**

- TXT/Markdown/HTML/EPUB automated parity is established;
- source-born structured EPUB sentence identity is integrated;
- native pretty rendering is bounded and responsive on the real large EPUB;
- pretty spoken-sentence highlighting and viewport follow are now accurate on the real Windows machine;
- text-only scroll follows correctly but its visible row highlight remains broken and is assigned to Goal 0009.

## P2.5 — First-class Caliberate service

**COMPLETE — GOAL 0008 CLOSED**

- Caliberate uses the versioned HTTP/JSON API at `127.0.0.1:8181`;
- existing library browser remains the UI;
- large catalog state is shared rather than deep-cloned per frame;
- supported formats materialize into the normal source/session/TTS pipeline;
- legacy Calibre compatibility remains behind the provider boundary;
- large real Caliberate EPUB opens quickly and is snappy;
- Windows speech, Play/Pause, voice selection, accurate pretty highlight, and synchronized viewport follow are verified end-to-end.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**READY — NEXT AUTHORIZED WORK**

Goal 0009 must preserve Goal 0008 and close four bounded residuals:

1. diagnose/eliminate unsolicited replay of a just-finished audio item across normal batch/refill/window progression;
2. make text-only visible highlight consume the same canonical ID its scroll-follow already uses;
3. make failed/unready Piper selection transactional and recoverable without reopening the book;
4. replace the current whole-AppConfig per-book cache ownership with explicit layered overrides.

Configuration precedence is fixed:

```text
compiled/platform defaults
-> app conf/config.toml
-> per-book reader overrides
-> live session
```

Windows app default must portably prefer **Zira** for new/unoverridden books. An explicit voice selected for one book persists as that book's stable Windows voice-ID override. Books without overrides continue inheriting the app preference.

Do not hardcode an opaque machine-specific Zira voice ID. Do not implement a full Piper model downloader/catalog as part of 0009.

## P3 — Native PDF visual stability

**NEXT CORE PRODUCT GATE AFTER GOAL 0009**

- page raster/render ownership;
- texture/cache lifecycle;
- viewport scheduling;
- zoom/scroll stability;
- bounded memory/performance on representative PDFs;
- visual behavior independent of TTS.

## P4 — PDF text/TTS/highlight synchronization

After P3:

- canonical sentence/page mapping;
- geometry confidence and overlays;
- jump/follow behavior;
- OCR/degraded confidence modes;
- first-sample playback identity integrated with PDF geometry;
- regression corpus for prior drift/jitter failures.

## P5 — Format expansion / ingestion hardening

- DOCX/Word;
- HTML edge-case hardening beyond current structured path;
- shared source/document boundaries;
- format-level regression fixtures.

## P6 — Ergonomics, latency, packaging

Only after foundations are trustworthy:

- startup/TTS latency optimization;
- UI cleanup;
- large-document ergonomics;
- Piper model/voice management if still desired;
- library/import polish;
- release packaging and dependency cleanup.