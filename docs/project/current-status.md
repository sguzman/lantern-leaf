# LanternLeaf Current Status

Updated: 2026-09-10 after director review of Goal 0009 A2.

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

**A2 IMPLEMENTATION DIRECTIONALLY GOOD; DIRECTOR REJECTED BEFORE HUMAN QA; A2.1 REQUIRED**

A1 implementation: `52ae85dad02f2e5588c14d33817abf0c5db69916`.

A1 real-desktop wins remain authoritative:

- sustained playback on the large EPUB had no unsolicited duplicate just-finished line reads;
- pretty rendering remained responsive and its audible-sentence highlight + viewport follow were correct;
- a new/unoverridden Windows book selected Zira;
- an explicit Mark selection persisted across restart/reopen;
- unavailable Piper produced an actionable missing-model failure without crashing.

A2 implementation: `e6bbc065462434950802818d0b4236464c244d5a`.

A2 worker terminal: `87ec9257948bbc8dff277c8a7d8c8b3d44ef3d31`.

A2 Windows CI: `34528119986` — both `native-workspace` and `hosted-renderer-probe` passed.

Director-reviewed A2 production changes are promising and should be preserved:

- Safe Quit now uses an ordered persistence-terminal -> egui native-close handshake rather than the old no-op/race;
- persistent top-chrome `Close book` confirms before destruction and sequences TTS stop, persistence, session close, and Starter return;
- stale playback events for a closed/different source are filtered;
- left panel width is bounded/resizable and long TTS/voice diagnostics wrap;
- text-only has a production-owned canonical row projection, mode-switch follow re-arming, and page-transition document refresh.

A2 is **not accepted** because the explicit regression gates that were designed to prevent another A1-style false positive are missing or incomplete:

- the new text-only test still proves only canonical->local arithmetic and does not exercise the real pretty->text-only transition, auto-scroll pending/consume lifecycle, subsequent SentenceStarted boundaries, Pause, or page transition;
- no app-level close-book lifecycle + persistence failure + stale-next-book isolation regression was supplied;
- Safe Quit has a planning test but no persistence-terminal -> native-close handshake regression;
- the 300+ character diagnostic/panel-width containment regression is absent;
- the existing failed-Piper test still stops after confirming backend remains Windows and never immediately calls Play/proves playback in the same ReaderSession.

The A2.1 correction contract is recorded in `docs/work/reviews/0009-a2-director-rejection.md`. No human QA is requested yet.

## Non-PDF reader status

**PRETTY EPUB PATH STRONG; TEXT-ONLY/EXIT RECOVERY A2.1 EVIDENCE OPEN**

TXT/Markdown/HTML/EPUB automated parity remains covered. The real large EPUB has strong evidence for responsive pretty rendering, stable Windows speech, no duplicate ordinary lines, accurate spoken-sentence highlight, and viewport follow. A2 contains plausible fixes for the remaining text-only and exit UX defects, but they must be protected by the required production-lifecycle regressions before another desktop pass.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY WAITS FOR GOAL 0009**

Gate 3 native PDF visual stability remains next but is not authorized until Goal 0009 closes.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0009 remains the active repository macro-goal. A2 terminalized on the existing branch but failed director acceptance because required deterministic gates were omitted. Start a fresh Codex Goal session as A2.1 on the same Goal 0009 branch/report lineage, synchronize the latest director review from `main`, re-arm the watcher, and continue without human QA until director acceptance.
