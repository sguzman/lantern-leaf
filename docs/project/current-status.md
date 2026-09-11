# LanternLeaf Current Status

Updated: 2026-09-10 after director acceptance/integration of Goal 0009 A2.1 for focused real-desktop signoff.

This file contains current verified/bounded state. Detailed historical correction lineage lives in `docs/work/reports/` and `docs/work/reviews/`.

## Workspace / architecture

**VERIFIED / AUTHORITATIVE**

- Native Rust + `eframe`/`egui` is the production desktop architecture.
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

Real Windows evidence proves audible Windows speech, Play/Pause, installed voice switching, and sentence-boundary-driven pretty synchronization.

## Goal 0008 / Gate 2.5 — Caliberate first-class library service

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

The large real Caliberate EPUB opens quickly, remains responsive, speaks through Windows TTS, highlights the actually audible sentence in native pretty view, and follows playback correctly. Caliberate remains behind the existing provider/browser boundary and legacy Calibre compatibility remains available.

## Goal 0009 / Gate 2.6 — TTS playback polish and layered voice configuration

**A2.1 ACCEPTED AND INTEGRATED — FOCUSED REAL-DESKTOP SIGNOFF PENDING**

A1 real-desktop wins remain authoritative:

- sustained playback on the large EPUB had no unsolicited duplicate just-finished line reads;
- pretty rendering remained responsive and its audible-sentence highlight + viewport follow were correct;
- a new/unoverridden Windows book selected Zira;
- an explicit Mark selection persisted across restart/reopen;
- unavailable Piper produced an actionable missing-model failure without crashing.

Accepted A2.1 implementation: `b5e348f06a1ff730d2363dc61bc4ac864d871f07`.

Accepted A2.1 worker terminal: `66091555e41f03fe2fbce049c8d773024af55425`.

Authoritative Windows CI: `34532877674` — both `native-workspace` and `hosted-renderer-probe` passed.

Director-accepted A2/A2.1 corrections:

- production text-only mode transition re-arms follow from the live canonical cursor and text-only row styling + scroll consume the same canonical projection;
- page transitions refresh the low-frequency document projection only when lightweight playback actually changes page;
- stale old-source playback events are rejected;
- left TTS/settings panel is resizable and bounded to 240–460 px with wrapped long diagnostics;
- persistent top-chrome Close book confirms before destruction and sequences TTS stop -> persistence -> CloseReaderSession -> Starter;
- Safe Quit sequences TTS stop -> persistence terminal success -> one native `ViewportCommand::Close`; persistence failure leaves the app open;
- failed Piper selection remains transactional and deterministic tests now prove immediate Windows Play plus first boundary/progress in the same ReaderSession;
- A1 continuation/no-repeat, Zira inheritance, per-book voice override, and Goal 0008 synchronization regressions remain green.

The acceptance record is `docs/work/reviews/0009-a2.1-director-acceptance.md`.

One focused desktop pass still must confirm visible text-only selection/follow, panel containment, same-session Piper recovery, Close book, and actual Safe Quit behavior before Goal 0009 closes.

## Non-PDF reader status

**PRETTY EPUB ACCEPTED; TEXT-ONLY/EXIT FIXES ACCEPTED FOR FINAL DESKTOP SIGNOFF**

TXT/Markdown/HTML/EPUB automated parity remains covered. The large real EPUB has strong evidence for responsive pretty rendering, stable Windows speech, no duplicate ordinary lines, accurate spoken-sentence highlight, and viewport follow. A2.1 now has deterministic coverage for the text-only transition/follow and exit lifecycles that were missing from A2.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY WAITS FOR GOAL 0009 FINAL SIGNOFF**

Gate 3 native PDF visual stability remains next but is not authorized until Goal 0009 closes.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0009 is integrated on `main` for focused human verification. Do not start another Codex Goal unless the real-desktop pass exposes a concrete remaining defect. If the pass succeeds, close Goal 0009 and authorize Gate 3 native PDF visual stability.
