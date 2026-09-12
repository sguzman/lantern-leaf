# LanternLeaf Current Status

Updated: 2026-09-12 after Goal 0012 A7 director acceptance for focused real-desktop signoff.

This file contains current verified/bounded state. Detailed attempt history lives in `docs/work/reports/` and `docs/work/reviews/`.

## Workspace / architecture

**VERIFIED / AUTHORITATIVE**

- Native Rust + `eframe`/`egui` is the production desktop architecture.
- Rust owns canonical document/session/playback state.
- Tauri/React/WebView is historical reference only.
- Windows human workflow is repo-native: `git pull -> .\qa.ps1`.
- Scoop is the active Windows CLI dependency convention.
- Heavy/blocking work must never run on the egui/render thread.

## Gate 0 — Windows baseline

**COMPLETE**

Hosted Windows CI covers MSVC/Pandoc setup, workspace check/build/test, repo-native QA preparation, Windows TTS synthesis/decode, and a separate hosted renderer probe.

## Gate 1 — backend-neutral TTS + Windows TTS

**COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted runtime shape:

`canonical display sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession cursor -> native UI projection`

Real Windows evidence proves audible Windows speech, Play/Pause, ordinary installed voice switching, and sentence-boundary-driven pretty/text-only synchronization.

## Goal 0008 / Gate 2.5 — Caliberate first-class reader integration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

The large real Caliberate EPUB opens quickly, remains responsive, speaks through Windows TTS, highlights the actually audible sentence in native pretty view, and follows playback correctly.

## Goal 0009 / Gate 2.6 — TTS playback polish and layered voice configuration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Stable audible TTS, no unsolicited duplicate ordinary lines, correct pretty/text-only spoken-sentence highlight/follow, portable Zira inheritance, per-book voice persistence, transactional Piper rejection/recovery, and accepted Close book / Safe Quit lifecycle are real-desktop accepted.

## Goal 0012 — pretty presentation controls and inline images

**A7 DIRECTOR-ACCEPTED — FOCUSED REAL-DESKTOP SIGNOFF PENDING**

Physical Windows QA after A5 already verified startup on the formerly crashing configuration, inline/cover EPUB imagery, the Presentation surface, multiple working visual controls, and synchronized canonical TTS identity/follow.

A6 implementation `6dcf9815ef87303caa3a3421bb8cc9e832f6b8ea` added the substantive presentation correction set:

- literal horizontal margins with no hidden 720-px text-column cap;
- vertical margin as viewport/frame inset rather than scroll-document padding;
- presentation geometry-key invalidation of measured pretty-block heights;
- vertically scrollable settings/presentation body;
- real-egui word/letter-spacing behavior;
- readable table/TOC widths with horizontal overflow;
- visible optional-font availability/effective fallback state;
- media sizing evidence while preserving inline image behavior.

A7 implementation `6fd324869ce6cca0c858c3ee32fabe627efba47b` closes the remaining director blockers:

- blockquote rule geometry is derived from the final measured quote frame rectangle rather than pre-layout `ui.max_rect()`;
- a real egui long-quote regression proves the rule remains within the quote and does not bleed into neighboring paragraphs;
- a stateful geometry-A -> geometry-B -> follow -> consume -> subsequent ordinary render-window regression runs across 64 canonical boundaries and proves the active target remains naturally renderable after the one-shot follow lifecycle is consumed;
- canonical playback/highlight ownership remains unchanged and no permanent target forcing was introduced.

Windows baseline run `34723376577` passed both native workspace and hosted renderer jobs, including workspace tests, Windows TTS, repository QA preparation, watcher policy, and hosted renderer capability gates.

One focused physical Windows pass is now authorized. Goal 0012 is not finally closed until that pass verifies literal margins, panel scrolling, word/letter spacing, readable TOCs/tables, restrained blockquotes, understandable font fallback, idle inline images, and continuously visible spoken-sentence pretty highlighting.

See `docs/work/reviews/0012-a7-director-acceptance.md`.

## Starter shell containment

**QUEUED AS GOAL 0013 — NOT ACTIVE**

Physical QA showed Recents and Browser Tabs/adjacent starter groups visually overlapping. This remains a separate responsive starter-shell containment goal and is not part of Goal 0012 signoff.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

The earlier `42866` open failure occurred while Caliberate was not running and remains withdrawn as evidence of a LanternLeaf materialization defect.

The remaining catalog issue is cover availability: main catalog entries can show black placeholders before open while Recents can show real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not authorized while Goal 0012 awaits physical signoff.

## Workflow status

**HUMAN QA GATE ACTIVE**

Goal 0012 A7 is worker-terminal, CI-green, integrated, and director-accepted for one focused desktop pass. No new Codex macro-goal is authorized until that pass is reviewed.

Goal 0010 and Goal 0013 remain queued; Goal 0011 remains deferred; PDF work remains future.
