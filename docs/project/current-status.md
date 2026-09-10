# LanternLeaf Current Status

Updated: 2026-09-10 after Goal 0009 A1 real-desktop partial pass and A2 correction authorization.

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

**A1 PARTIAL REAL-DESKTOP PASS — A2 CORRECTION READY**

A1 implementation: `52ae85dad02f2e5588c14d33817abf0c5db69916`.

A1 worker terminal: `fcbe092cdf543e8095ce73c8317c3a777b68d88e`.

A1 Windows CI: `34517286850`.

Real-desktop A1 PASS evidence to preserve:

- sustained playback on the same large EPUB produced no unsolicited duplicate just-finished line reads;
- pretty view remains fast/responsive and its audible-sentence highlight + viewport follow remain correct;
- a new/unoverridden Windows book selected Zira;
- an explicit Mark voice selection persisted across restart/reopen;
- unavailable Piper produced an actionable missing-model error without crashing the app.

Real-desktop A1 FAIL / A2 correction scope:

- pretty -> text-only during active playback has neither visible highlight nor auto-scroll, regressing the earlier follow-working state;
- long Piper error/path text can force the left TTS/settings panel excessively wide and make resizing unusable until content changes;
- Safe quit confirmation does not exit the application; director audit confirms the egui `handle_safe_quit` path is currently a logging-only no-op;
- leaving the current book is not discoverable/reliable: existing reader controls are buried in content and `Close reader session` dispatches close before showing its confirmation;
- same-session Windows playback recovery after failed Piper still needs deterministic + eventual real-desktop proof.

A2 is authorized on the same Goal 0009 branch/report lineage. It must repair production text-only highlight/follow, constrain diagnostics layout, provide a persistent ordered Close book/Back to library action, make Safe Quit actually close only after ordered persistence, and prove failed-Piper -> Windows playback recovery in the same session.

## Non-PDF reader status

**PRETTY EPUB PATH STRONG; TEXT-ONLY A2 CORRECTION OPEN**

TXT/Markdown/HTML/EPUB automated parity remains covered. The large real EPUB now has repeated real-desktop evidence for responsive pretty rendering, stable Windows speech, no duplicate ordinary lines, accurate spoken-sentence highlight, and viewport follow. Text-only remains the blocking non-PDF presentation defect because its actual production mode transition currently loses both highlight and follow.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY WAITS FOR GOAL 0009 A2**

Gate 3 native PDF visual stability remains next but is not authorized until Goal 0009 closes.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0009 is reopened to `docs/work/ready/` for a fresh Codex Goal session as Attempt A2. The repository goal ID, implementation branch, and report lineage remain 0009. No human QA is requested during A2 implementation. PDF work remains unauthorized.
