# 0010 — Caliberate catalog reliability: materialization diagnostics and first-class covers

## Outcome

Make the Caliberate-backed library trustworthy before a book is opened: catalog covers should be available through a first-class provider path, and failed book materialization should expose enough stage/format/provider evidence to diagnose and recover deterministically.

## Why now

Real desktop evidence after Goal 0009 A2.1 showed two provider/catalog defects distinct from TTS:

- Caliberate book `42866` failed before reader open with `calibre_open_failed: failed to materialize Caliberate book 42866`, but the top-level failure does not expose format/HTTP/stage detail;
- Caliberate catalog entries render black cover placeholders, while the same books can acquire real covers after being opened and subsequently appear correctly in Recents.

Current LanternLeaf behavior explains the cover asymmetry: the catalog starts without a local materialized EPUB, while Recents can extract the embedded cover from the now-local source. The Caliberate provider does not currently have a first-class catalog-cover fetch path in LanternLeaf.

## Authorized passes

### A — inspect the Caliberate API contract before implementation

Inspect the current `sguzman/caliberate` server/API rather than guessing legacy Calibre routes.

If Caliberate has no first-class cover endpoint/URL contract, either:

- coordinate the smallest explicit Caliberate-side API addition, such as a book cover endpoint or cover URL field; or
- terminalize blocked with that concrete cross-repository dependency if this Goal execution is not authorized to change Caliberate.

Do not probe invented legacy thumbnail URLs.

### B — actionable materialization diagnostics

A failed catalog open must retain provider context including, where available:

- Caliberate book ID and title;
- selected requested format;
- materialization stage;
- endpoint/path class without leaking secrets;
- HTTP/status or transport failure category;
- validation failure category for downloaded bytes;
- underlying error/source chain.

The UI should return from SourceLoading to a recoverable error state without corrupting the previous/new reader lifecycle.

### C — deterministic format recovery

If catalog metadata and server format availability disagree, recovery may try another format only when it is explicitly advertised/allowed and supported by LanternLeaf.

Do not silently guess arbitrary formats. Preserve provider ordering/policy and log the attempted sequence.

### D — first-class lazy catalog covers

For Caliberate books with cover availability:

- fetch covers lazily for visible/near-visible catalog rows only;
- perform network/disk/decode work off the egui render thread;
- cache successful cover bytes/textures and negative results with sane invalidation;
- bound in-flight requests/concurrency;
- never materialize/download the full EPUB merely to obtain a thumbnail;
- never issue cover requests for all ~100k catalog books at startup;
- preserve Recents/local embedded-cover fallback where appropriate.

The main catalog and Recents may share cache artifacts once available, but should not rely on a book having been opened first.

## Acceptance gates

1. A Caliberate materialization failure exposes provider/book/format/stage/root-cause diagnostics sufficient to distinguish transport, HTTP, unsupported/missing format, invalid payload, and local materialization failure classes.
2. Format fallback, if any, is limited to explicitly advertised and LanternLeaf-supported formats.
3. A project-owned failure fixture/regression proves SourceError is recoverable and does not poison later opens.
4. Catalog cover loading uses an explicit Caliberate provider contract, not legacy route guessing or full-book download.
5. Visible-row/lazy cover scheduling is bounded and off the UI thread; a 100k catalog does not create unbounded cover work.
6. A catalog book can display a cover before it has ever been opened/materialized.
7. Recents/local cover behavior remains working.
8. Existing Goal 0008/0009 large EPUB/TTS behavior remains green.
9. Windows CI and normal workspace validation pass.

## Non-goals

- text-only/TTS fixes from Goal 0009;
- Windows Natural/HD voice support;
- PDF implementation;
- broad library UI redesign.

## Repository handoff

This goal remains queued until Goal 0009 closes and the director promotes exactly one ready goal. Use a new `codex/0010-caliberate-catalog-reliability` branch when authorized.
