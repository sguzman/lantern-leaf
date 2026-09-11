# Goal 0009 — A3 director acceptance

Status: **A3 ACCEPTED AND INTEGRATED — REAL-DESKTOP SOURCE-DEPENDENT TEXT-ONLY SIGNOFF PENDING.**

Implementation: `8975cfcb286508e19ac1a983b3e49d35b83d38cf`.

Worker terminal: `7f5f3f77538ecd5fd0acca938f86402003224b2e`.

Authoritative Windows CI: `34558938955` — green.

## Director findings

A3 fixes the production ownership defect exposed by the real-desktop disagreement between `A General History and Collection of Voyages` and `Buffalo Bill`.

- `ReaderSession::current_sentences()` now always projects stable `raw_page_sentences`; text-only presentation no longer calls `ensure_current_plan()` or uses `TtsNormalizationPlan.audio_sentences` as document rows.
- `current_highlight_idx()` now remains display/canonical-owned instead of substituting `highlighted_audio_idx` in text-only mode.
- Text-only sentence clicks select a canonical/display row first and only then derive an audio mapping for playback.
- Reader stats and idle/paused TTS view construction no longer force a normalization plan merely to render/snapshot the document.
- Snapshot TTS current-sentence text reads only an already-existing plan instead of constructing one as a UI side effect.
- Source-shaped core regressions cover stable rows with no plan, awkward/metadata-heavy canonical text whose audio cardinality differs, plan replacement/removal, stopped/paused playback, Play/Resume, and return to pretty mode.
- The existing production text-only projection/follow regression continues to exercise 48+ canonical transitions and page changes, while A3's core regression now proves those projections are fed by source-stable canonical rows rather than transient audio rows.

This satisfies the A3 acceptance intent: text-only is once again a document presentation mode, while normalization/audio chunking remains an internal synthesis concern.

## Validation

Windows run `34558938955` passed:

- repo-native QA preparation;
- staged Windows QA configuration and synthesis;
- workspace check;
- workspace build;
- workspace tests;
- Windows TTS diagnostic probe;
- hosted renderer probe.

Worker validation also reports `cargo test --workspace -- --test-threads=1`, `cargo check --workspace`, and `git diff --check` passing. Windows Natural/Narrator/HD voices were intentionally untouched and remain outside Goal 0009.

## Remaining human evidence

Only the source-dependent desktop behavior needs re-verification:

1. open `A General History and Collection of Voyages`;
2. switch to text-only before or during playback and confirm text is present immediately;
3. confirm spoken sentence highlight and auto-follow work through several boundaries;
4. repeat on `Buffalo Bill` to ensure the previously working source remains working;
5. switch back to pretty view briefly and verify the already-accepted pretty synchronization remains intact.

No Natural/Narrator voice testing is part of this signoff.

If both EPUBs pass, Goal 0009 can close. The separate Caliberate materialization/catalog-cover defects remain queued as Goal 0010. Windows Natural/Narrator/HD voice work remains deferred until the user explicitly re-authorizes that surface.
