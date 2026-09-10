# LanternLeaf Current Status

Updated: 2026-09-10 after director acceptance of Goal 0009 A1 for real-desktop QA.

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

Real Windows evidence already proves audible Windows speech, Play/Pause, interactive installed-voice changes, and sentence-boundary-driven pretty synchronization.

## Goal 0008 / Gate 2.5 — Caliberate first-class library service

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

The large real Caliberate EPUB now opens quickly, remains responsive, speaks through Windows TTS, highlights the actually audible sentence in native pretty view, and follows playback correctly. Caliberate remains behind the existing provider/browser boundary and legacy Calibre compatibility remains available.

Final Goal 0008 acceptance is recorded in `docs/work/reviews/0008-a8.3-director-acceptance.md`.

## Goal 0009 / Gate 2.6 — TTS playback polish and layered voice configuration

**A1 IMPLEMENTATION ACCEPTED — REAL-DESKTOP SIGNOFF PENDING**

Accepted implementation: `52ae85dad02f2e5588c14d33817abf0c5db69916`.

Accepted worker terminal head: `fcbe092cdf543e8095ce73c8317c3a777b68d88e`.

Authoritative Windows CI: `34517286850`.

Implemented and director-reviewed:

- `[tts].windows_voice_preference = "Zira"` is the portable app-level Windows preference;
- explicit installed per-book Windows voice IDs win over the app preference;
- absent/invalid preferred-name availability falls back to the Windows OS default rather than making speech unusable;
- versioned `BookReaderOverrides` replaces whole-AppConfig book ownership for new persistence;
- omitted book fields inherit current app configuration on every open;
- legacy whole-AppConfig book caches migrate reader-local fields without inventing backend/voice override intent;
- explicit reader TTS settings update book override intent while global runtime/resource settings remain app-owned;
- TTS refill/window progression carries an explicit continuation cursor; a deterministic 300-boundary regression crosses repeated 8-item batches and 64-sentence windows with exact ordered ordinary starts and no duplicate boundary IDs;
- text-only row selection now consumes the same high-frequency canonical playback identity as scroll follow;
- real backend/voice changes are validated before session mutation; invalid/unready Piper selection leaves the last-known-good Windows configuration intact and emits an actionable failure;
- Goal 0008 canonical session, source-born EPUB identity, first-sample boundaries, bounded rendering, and lightweight hot paths remain intact.

One real-desktop pass is still required because CI cannot prove audible duplicate absence, visible text-only styling, actual installed Zira selection, book-level reopen behavior through the GUI, or same-session recovery after a failed Piper attempt.

## Non-PDF reader status

**PRETTY PATH ACCEPTED; GOAL 0009 DESKTOP POLISH SIGNOFF PENDING**

TXT/Markdown/HTML/EPUB automated parity remains covered. The large real EPUB already provides strong evidence for native pretty rendering, scrolling, Windows speech, and spoken-sentence synchronization. Goal 0009's accepted A1 patch targets the remaining text-only visual and TTS/configuration polish; desktop signoff is the only remaining Gate 2.6 evidence.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY WAITS FOR GOAL 0009 SIGNOFF**

Earlier work repaired bounded PDF classification/OCR/reading-order/cache contracts. Gate 3 native PDF page rendering/viewport/texture stability is next, but it is not authorized until Goal 0009's bounded real-desktop signoff completes.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0009 is integrated for human verification. No next macro-goal is authorized yet. If desktop QA passes, director closes 0009 and opens the PDF visual-stability goal. If it fails, the same Goal 0009 lineage is reopened for a bounded correction.
