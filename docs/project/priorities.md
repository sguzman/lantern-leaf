# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

Reproducible Windows build/check/test, native egui launch, repo-native `deps.ps1` / `qa.ps1`, Windows CI, separate renderer probe, and Scoop dependency convention are established.

## P1 — Backend-neutral TTS + Windows speech

**WORKING WINDOWS PATH COMPLETE**

Canonical reader/session semantics are backend-neutral; ordinary WinRT Windows voice enumeration/synthesis/playback works; first-sample boundaries drive canonical playback identity; physical Windows speech and interactive ordinary voice changes are verified.

## P2 — Non-PDF reader/TTS

**PRETTY EPUB ACCEPTED; GOAL 0009 A3 AUTOMATED ACCEPTED, FINAL TEXT-ONLY DESKTOP SIGNOFF PENDING**

- TXT/Markdown/HTML/EPUB ingestion/parity remains covered;
- native pretty rendering is bounded, responsive, and sentence-synchronized on the real large EPUB;
- ordinary Windows TTS no longer shows the prior duplicate-line refill bug in desktop QA;
- A3 decouples text-only visible rows from transient TTS audio-plan ownership;
- text-only row/highlight semantics are now canonical/display-owned regardless of zero/one/many audio chunks or plan availability;
- final evidence is the same two real EPUBs that previously disagreed.

## P2.5 — First-class Caliberate reader integration

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**A3 ACCEPTED — HUMAN SIGNOFF PENDING**

A3 implementation `8975cfcb286508e19ac1a983b3e49d35b83d38cf` / Windows CI `34558938955` is director-accepted.

Final human check:

1. `A General History and Collection of Voyages`: text-only contains text immediately and highlights/follows spoken canonical sentences;
2. `Buffalo Bill`: previously working text-only behavior remains working;
3. a short pretty-view playback sanity check remains correct.

If those pass, close Goal 0009.

## P2.7 — Goal 0010: Caliberate catalog covers + provider availability UX

**QUEUED — NEXT CANDIDATE AFTER GOAL 0009 CLOSES**

Corrected evidence: the earlier `42866` open failure occurred while Caliberate itself was not running. Do not treat that incident as evidence of a LanternLeaf materialization/format bug.

Goal 0010 should:

- distinguish provider-unavailable/connection failure from book/content failure;
- recover cleanly once Caliberate returns;
- add a first-class Caliberate cover contract if the server lacks one rather than probing invented legacy routes;
- load covers lazily for visible rows, off the GUI thread, with bounded concurrency/cache;
- never download/materialize all ~100k books or full EPUBs just for thumbnails;
- make catalog covers available before a book has been opened while preserving Recents/local cover fallback;
- replace unexplained black rectangles with intentional loading/no-cover/provider-error states.

Materialization hardening/format fallback is **not** authorized merely because of the withdrawn offline-provider incident.

## P2.8 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate, implement, or test Natural/Narrator/HD voices for now. Preserve the existing ordinary Windows voice backend and current Zira/per-book override behavior. The user will explicitly notify the project when this surface is ready to resume.

## P3 — Native PDF visual stability

**FUTURE CORE PRODUCT GATE; NOT AUTHORIZED UNTIL GOAL 0009 CLOSES**

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
