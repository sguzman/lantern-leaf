# LanternLeaf Failure and Dead-End Catalog

This file records approaches that failed, partially failed, or produced misleading confidence. It exists to prevent future agents from rediscovering the same traps and to preserve the reasoning behind current architecture.

A failure entry is not an embarrassment. It is a constraint discovered by contact with the real product.

## 1. Web/Tauri/WebView ownership as the desktop authority

### What went wrong

The historical desktop stack accumulated browser/WebView lifecycle and rendering machinery that made state ownership hard to reason about and could produce severe interaction latency.

### Important physical evidence

The native rewrite itself initially reproduced catastrophic lag in EPUB presentation, proving that "native" alone is not enough. But the old web stack still remained the wrong authority because it added hidden lifecycle/rendering layers on top of the reader's already difficult synchronization problem.

### Durable conclusion

Native Rust + egui is authoritative. Historical web/Tauri code may be consulted for feature intent, not restored as the controlling architecture.

## 2. Assuming build success means platform success

### What went wrong

Early Windows recovery could compile while native launch/test/runtime truth remained incomplete.

### Durable conclusion

Windows is accepted only through layered evidence: compile/check/test/CI plus real desktop behavior where applicable.

## 3. Runtime bindgen/libclang dependency for ordinary Windows builds

### What went wrong

Native dependency setup made the build depend on local bindgen/libclang availability.

### Resolution

Goal 0001 removed runtime bindgen/libclang requirements and hardened vendored bindings.

## 4. Treating TTS request/queue time as spoken ownership

### What went wrong

Visual state can get ahead of actual audio when highlight advances at synthesis or queue time.

### Resolution

Canonical playback ownership moved to the first audible sample boundary.

### Never regress

The sentence whose first sample begins is the sentence that owns canonical spoken progress.

## 5. Confusing provider-unavailable with corrupt materialization

### What went wrong

A Caliberate book failure looked like a bad book/materialization issue.

### Physical truth

Book `42866` failed because Caliberate was not running.

### Resolution

Provider availability became an explicit failure class.

## 6. Materializing an EPUB just to get its cover

### What went wrong

The materialization path was too expensive and semantically wrong for thumbnail retrieval.

### Resolution

Caliberate gained a narrow `/api/v1/books/{id}/cover` endpoint; LanternLeaf fetches covers lazily and independently.

## 7. Unbounded or overlapping catalog work

### What went wrong

A first library continuity attempt risked overlapping provider walks, masking failures, and overwriting live cover state during final reconciliation.

### Resolution

Progressive batch publication, one authoritative walk, explicit completion/error state, and preservation of live per-book state.

## 8. Treating true cold-cache cost as a warm regression

### What went wrong

After resetting QA state, first open could take roughly 5–7 seconds and initially looked suspicious.

### Resolution

Classify cold and warm behavior separately. Warm reopen remained immediate; reset intentionally removes cache/materialized/derived state.

## 9. Native egui assumed to be automatically fast

### What went wrong

The EPUB reader became so laggy that hover feedback could take seconds.

### Resolution

Move expensive work off the render thread, reduce rebuild scope, bound layout/image work, and treat physical frame responsiveness as an architectural property.

## 10. Hidden 720px content clamp

### Symptom

Huge empty gutter despite available width.

### Cause

A hidden width clamp survived into the native presentation path.

### Resolution

Literal presentation geometry became authoritative and responsive to actual available center width.

## 11. Reflow anchor recaptured from unstable intermediate geometry

### Symptom

Violent scroll jumps while changing font/spacing/presentation settings.

### Failed idea

Recapture the viewport witness every frame through an ongoing geometry transition.

### Why it failed

Intermediate geometry is not a stable reference frame.

### Resolution

Capture once from last-stable geometry, retain one semantic witness through the reflow transaction, reconcile after new geometry is measured, and yield to explicit user scrolling / pending TTS follow according to precedence rules.

## 12. Quack-check as a potential baseline PDF dependency

### What made it attractive

It already had classification, transcript orchestration, chunking, reports, quality tiers, and deterministic artifacts.

### Why it was rejected as baseline

It carries Python/Docling/OCR/pypdf assumptions, subprocess/timeouts, environment discovery, and even historical hardcoded path assumptions. Making visual open depend on that stack would make basic PDF reading fragile.

### Durable conclusion

Quack-check is archaeology/source material. Native visual Pdfium is tier 0. Native embedded text is tier 1. Hostile recovery is later, typed, optional, and subordinate.

## 13. Whole-document PDF text extraction as one uninterrupted Pdfium job

### Attempt

Goal 0022 A1.

### Failure

One text job could monopolize the single Pdfium owner for hundreds of pages, starving visible raster work.

### Resolution direction

Cooperative bounded chunks/pages with arbitration between units. Any queued raster work outranks background text extraction.

## 14. Document-scale text adoption on egui

### Attempt

Goal 0022 A1.

### Failure

Sentence splitting, counts/indexes, cache-hint construction, and cache persistence were performed from the UI commit path.

### Resolution

Prepare immutable document-scale text state off-thread. The UI commit validates identity and swaps prepared state only.

## 15. Trusted text adopted without promoting capability policy

### Attempt

Goal 0022 A2.

### Failure

The document could internally contain trusted text while production runtime policy still said Text-only/search/TTS were disabled.

### Resolution

Trusted adoption must atomically promote the canonical PDF capability policy while still keeping exact visual sentence sync disabled until geometry exists.

## 16. "Bounded" UI commit still creating a full ReaderSnapshot

### Attempt

Goal 0022 A2/A3 lineage.

### Failure

A document-scale snapshot flattened/cloned canonical sentences during adoption.

### Resolution

Explicit enriched-PDF bounded projection: current-page-local strings plus Arc clones of immutable shared document state; no whole-document assembly on egui.

## 17. Prioritizing only Current raster work over text extraction

### Attempt

Goal 0022 A2.

### Failure

Nearby pages can be simultaneously visible at continuous-page seams. Treating them as background allowed text extraction to delay actual visible content.

### Resolution

Any queued raster work is serviced before the next text unit, while the scheduler retains Current-over-Nearby ordering.

## 18. Reopening/reparsing the PDF once per extracted text page

### Attempt

Goal 0022 A2.

### Failure

A 638-page PDF could cause hundreds of document opens.

### Resolution

Bounded multi-page chunks or owner-local resumable work that yields frequently without document-open amplification.

## 19. Global PDF sentence identity falling back to legacy pre-enrichment counts

### Attempt

Goal 0022 A5.

### Failure

Later-page canonical IDs could collapse to page-local identity and confuse TTS continuation/final exhaustion/bookmarks.

### Resolution

Once trusted native text is adopted, every global identity path uses the prepared trusted document's prefix indexes and provenance.

## 20. Search query frozen at worker-preparation time

### Attempt

Goal 0022 A5.

### Failure

A query entered before enrichment could remain visible but have no matches after text adoption until manually resubmitted.

### Resolution

Reconcile the **current live query** after trusted document adoption, with source/generation/query-revision stale safety, off-thread.

## 21. One OS thread per retired enriched PDF snapshot

### Attempt

Goal 0022 A6.

### Why it looked reasonable

Large shared payload destruction should not occur on egui.

### Why it was wrong

Continuous page ownership/snapshot churn could create many short-lived threads, contaminating the exact path whose responsiveness had already been physically proven.

### Resolution

One bounded process-wide PDF retirement worker/queue.

## 22. Hidden O(total-pages) work inside a "bounded" projection

### Attempt

Goal 0022 A6.

### Failure

Global ID and before/after-page checks still scanned page-count vectors.

### Resolution

Prepared prefix sums for O(1) page bases and totals; binary search for global-sentence -> page mapping.

## 23. Natural PDF TTS continuation inferred rather than made explicit

### Physical failure

Goal 0022 A7 read a one-sentence title/front-matter page and stopped. Later pages sometimes crossed, making the behavior look inconsistent.

### Cause

The runtime knew more canonical text existed but rebuilt another bounded plan without first moving the authoritative ReaderSession to the next non-empty native page.

### Resolution direction

Explicit next-non-empty-native-page transition before rebuilding the TTS plan.

## 24. Search-panel visibility tied to one-shot focus request

### Physical failure

Search appeared and immediately disappeared before typing was possible.

### Cause

`pending_search_focus` was overloaded as both panel visibility and focus acquisition.

### Resolution direction

Persistent `search_panel_open` plus separate one-shot focus request.

## 25. Relative asynchronous Text-only toggle

### Physical failure

Rapid Pretty/Text-only toggling on EPUB eventually left only a fragment of the book rendering.

### Cause

Optimistic desired state + stale asynchronous snapshots + relative `ToggleTextOnly` could queue inversions.

### Resolution direction

Idempotent `SetTextOnly { enabled }`; same-target no-op; latest target authoritative.

## 26. Search shortcut suppression tied to requested focus, not actual focus

### Attempt

Goal 0022 A8.

### Failure found in source review

After `TextEdit::request_focus()` consumed the one-shot flag, the editor could still own keyboard focus while shell focus ownership reverted to Reader. Typing query characters could also fire reader shortcuts.

### Resolution required

Actual editor/egui keyboard focus controls shortcut suppression; merely requesting focus does not.

## 27. Search text reconstructed from asynchronously acknowledged reader state

### Attempt

Goal 0022 A8.

### Failure found in source review

Each frame cloned the last acknowledged canonical query into a temporary string. Fast typing could be replaced by an older acknowledgement.

### Resolution required

Persistent synchronous app-owned draft buffer; async canonical search acknowledgement/reconciliation stays separate and stale-safe.

## 28. Testing a runtime bug by manually calling its helper

### Attempt

Goal 0022 A8.

### Failure of evidence

The page-crossing regression called `advance_tts_to_next_non_empty_page()` directly rather than driving `run_tts_runtime_loop()` through simulated first-sample boundaries.

### Lesson

A helper unit test is not proof that the production control flow invokes the helper at the right time.

## 29. Comparing document-global canonical ID with page-local TTS plan boundary

### Attempt

Goal 0022 A8.

### Failure

Later pages can have global IDs far larger than local normalization-plan bounds, causing needless invalidation/rebuild.

### Correct coordinate discipline

Global ID owns document identity. Page-local index owns page-local normalization-plan membership.

## 30. Assuming the cache should visibly contain rendered PDF page images

### Confusion

A PDF could reopen quickly even though no forest of rendered pages appeared in the cache.

### Explanation

Rendered native page textures are intentionally ephemeral/bounded in memory. Durable source materialization and native text artifacts are separate concerns.

## 31. Repeated QA in one PowerShell process

### Symptom

Eventually `VsDevCmd.bat` can fail with `The input line is too long`.

### Likely mechanism

Repeated environment import accumulates Visual Studio variables into the same process.

### Current workaround

Fresh PowerShell for QA.

### Future owner

Goal 0018.

## 32. Warm catalog thumbnail hydration scanning the entire library

### Symptom

Warm runs showed multiple thumbnail-hydration scans walking roughly 104k rows, hitting multi-second budgets and rewriting large catalog cache state.

### Why it matters

The catalog scale makes "small" O(total-books) work product-visible.

### Future owner

Goal 0017: indexed/lazy thumbnail association, bounded retry/backoff, less cache/log churn.

## 33. Fit mode followed by manual +/- using stale manual ladder

### Symptom

After Fit Width/Fit Page, +/- jumps according to the previously remembered manual zoom instead of stepping from the current effective fit percentage.

### Severity

Minor polish; does not invalidate Goal 0020.

### Future owner

Goal 0021.

## Failure-review rule

When a new defect appears, classify it before changing code:

- architecture violation;
- stale identity / coordinate-domain bug;
- asynchronous state race;
- UI draft/focus ownership bug;
- provider capability/availability issue;
- cold-vs-warm behavior;
- physical-only rendering/performance defect;
- evidence gap where tests do not execute production control flow.

Many LanternLeaf failures became expensive because the first diagnosis was at the wrong layer.