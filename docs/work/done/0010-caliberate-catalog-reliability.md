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

Determine whether Caliberate already exposes cover bytes, a cover endpoint, a stable cover URL/path field, or cover-presence metadata.

If no first-class cover contract exists, coordinate the smallest explicit Caliberate-side API addition if authorized, or terminalize blocked with the exact cross-repository dependency. Do not probe invented legacy thumbnail URLs.

### B — provider availability state

A Caliberate service that is not running/unreachable is not a per-book materialization defect.

Requirements:

- detect/report provider-unavailable/connection-refused/timeout distinctly from a book-format/content failure;
- keep the library shell recoverable;
- retrying after Caliberate becomes available must work without restarting LanternLeaf where practical;
- diagnostics should identify the provider and availability class without dumping secrets or giant traces into the UI.

### C — first-class lazy catalog covers

For Caliberate books with cover availability:

- fetch covers lazily for visible/near-visible catalog rows only;
- perform network/disk/decode work off the egui render thread;
- cache successful cover bytes/textures and negative results with sane invalidation;
- bound in-flight requests/concurrency;
- never materialize/download the full EPUB merely to obtain a thumbnail;
- never issue cover requests for all ~100k catalog books at startup;
- preserve Recents/local embedded-cover fallback where appropriate.

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
9. Existing Goal 0008/0009 large EPUB/TTS behavior and Goal 0012–0014 reader/presentation behavior remain green.
10. Windows CI and normal workspace validation pass.

## Director correction A6

A5 is rejected before human QA; see `docs/work/reviews/0010-a5-director-rejection.md`.

Preserve A5's explicit Caliberate cover route, `has_cover` propagation, bounded visible-row scheduling, off-render-thread request/decode work, and intentional placeholder UI, but correct its per-cover completion ownership:

- do not use the single full-catalog `OperationScope::CalibreLoad` boolean as ownership for independent concurrent thumbnail requests;
- every started cover request must produce an explicit book-identified terminal outcome and leave the pending set;
- provider unavailable, endpoint/no-cover, and fetch/decode failure must not remain stuck as `Loading cover…`;
- transient provider failure must remain retryable without permanent negative poisoning;
- synchronize current LanternLeaf `main` before replaying A5 so the accepted Goal 0014 closure and Goal 0015 queue state are retained;
- synchronize current `sguzman/caliberate` `main`, replay the minimal cover endpoint, add targeted post-change cover-route/content tests, and run the relevant Caliberate suite after the change;
- wait for required LanternLeaf Windows CI success before terminal signaling.

Do not request human QA during A6. The director reviews both repository branches first.

## Non-goals

- speculative format fallback/materialization hardening based on the withdrawn offline-provider incident;
- reflow/highlight polish queued as Goal 0015;
- Windows Natural/HD voice support;
- PDF implementation;
- broad library UI redesign.

## Repository handoff

Use branch `codex/0010-caliberate-catalog-reliability`.

Synchronize current director `main`, move this goal `ready -> active`, re-arm the watcher, inspect the real Caliberate API contract first, then implement only the smallest explicit cross-repository/provider changes required. Run repository and Windows gates, write/update `docs/work/reports/0010.md`, terminalize only on full success or a true cross-repository escalation, push before signaling terminal state, and restore the shared checkout to `main`.

Do not request human QA during implementation. The director will review the pushed branch first.
