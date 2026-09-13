# 0010 — Caliberate catalog covers and provider-availability UX

## Outcome

Make the Caliberate-backed library trustworthy before a book is opened: catalog covers should be available through a first-class provider path, while ordinary provider-unavailable states should be reported clearly without being misdiagnosed as book materialization corruption.

## Corrected evidence

A previous desktop pass appeared to show Caliberate book `42866` failing during materialization. The user later clarified that **Caliberate was not running during that attempt**. That incident is therefore withdrawn as evidence of a LanternLeaf materialization/format defect.

Do not preserve or reintroduce a fake requirement to harden materialization based on that incident.

The remaining real catalog defect is independent and reproducible:

- Caliberate catalog entries can render black cover placeholders before a book has ever been opened;
- the same book can later display a real cover in Recents after the EPUB has been materialized locally and its embedded cover becomes available;
- this proves main-catalog cover presentation and Recents/local-cover presentation are using different availability paths.

The goal is to give the Caliberate catalog a first-class cover contract so books do not need to be opened/materialized before their covers can appear.

## Authorized passes

### A — inspect the actual Caliberate API contract

Inspect the current `sguzman/caliberate` server/API rather than guessing legacy Calibre routes.

Determine whether Caliberate already exposes:

- cover bytes;
- a cover endpoint;
- a stable cover URL/path field;
- cover presence metadata.

If no first-class cover contract exists, coordinate the smallest explicit Caliberate-side API addition if authorized, or terminalize blocked with the exact cross-repository dependency.

Do not probe invented legacy thumbnail URLs.

### B — provider availability state

A Caliberate service that is not running/unreachable is not a per-book materialization defect.

Requirements:

- detect/report provider-unavailable/connection-refused/timeout distinctly from a book-format/content failure;
- keep the library shell recoverable;
- retrying after Caliberate becomes available must work without restarting LanternLeaf where practical;
- diagnostics should identify the provider and availability class without dumping secrets or giant traces into the UI.

This is bounded provider UX, not speculative materialization redesign.

### C — first-class lazy catalog covers

For Caliberate books with cover availability:

- fetch covers lazily for visible/near-visible catalog rows only;
- perform network/disk/decode work off the egui render thread;
- cache successful cover bytes/textures and negative results with sane invalidation;
- bound in-flight requests/concurrency;
- never materialize/download the full EPUB merely to obtain a thumbnail;
- never issue cover requests for all ~100k catalog books at startup;
- preserve Recents/local embedded-cover fallback where appropriate.

The main catalog and Recents may share cache artifacts once available, but the catalog must not depend on a book having been opened first.

### D — black-placeholder semantics

A missing/not-yet-loaded cover should have an intentional placeholder state rather than an unexplained black rectangle.

Differentiate at least:

- cover loading;
- no cover advertised/available;
- provider unavailable;
- cover fetch/decode failure.

Do not allow a transient provider failure to poison the negative-cover cache permanently.

## Acceptance gates

1. The withdrawn `42866` incident is not treated as evidence of a materialization-format bug.
2. Provider unavailable/connection failure is classified separately from book/content/materialization failure.
3. A provider-unavailable state is recoverable once Caliberate returns, without corrupting reader/library state.
4. Catalog cover loading uses an explicit Caliberate provider contract, not legacy route guessing or full-book download.
5. Visible-row/lazy cover scheduling is bounded and off the UI thread; a ~100k catalog does not create unbounded cover work.
6. A catalog book can display a cover before it has ever been opened/materialized.
7. Recents/local cover behavior remains working.
8. Placeholder/loading/no-cover/provider-error states are visually intentional and distinguishable enough for debugging/UX.
9. Existing Goal 0008/0009 large EPUB/TTS behavior remains green.
10. Windows CI and normal workspace validation pass.

## Non-goals

- speculative format fallback/materialization hardening based on the withdrawn offline-provider incident;
- text-only/TTS fixes from Goal 0009;
- Windows Natural/HD voice support;
- PDF implementation;
- broad library UI redesign.

## Repository handoff

This goal remains queued until Goal 0009 closes and the director promotes exactly one ready goal. Use a new `codex/0010-caliberate-catalog-reliability` branch when authorized.
