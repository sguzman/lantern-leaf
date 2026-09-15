# LanternLeaf Thorny Issues and Open Risks

This document is not a roadmap. It records problems that are easy to misunderstand because they sit at subsystem boundaries, have misleading symptoms, or have already consumed multiple failed attempts.

Use it when an apparently local bug starts pulling on several parts of the reader at once.

## 1. Native PDF text/TTS remains the active hard problem

Goal 0022 is still open.

The visual PDF reader is already physically good. Do not destabilize it while fixing text/TTS/search.

Known accepted foundation:

- continuous visual scrolling is fast;
- transient raster placeholders converge quickly;
- visual PDF open is independent of text;
- Text-only can be populated from trusted embedded text;
- one Pdfium owner handles metadata/raster/text;
- extraction is cooperative;
- raster work has priority;
- immutable document-scale text state is shared;
- prefix indexes make projection bounded;
- large shared payload destruction uses one bounded retirement worker.

Active A9 risk areas at time of writing:

- actual Search text-editor focus must own shortcut suppression;
- Search draft must be synchronous and cannot be reconstructed from delayed canonical acknowledgement;
- runtime-level PDF TTS continuation must be proven through the real simulated runtime loop;
- document-global sentence IDs must not be compared with page-local TTS-plan bounds.

Do not mistake missing PDF pretty-surface sentence highlight for a Goal 0022 regression. Exact native PDF sentence geometry/highlight/follow is intentionally later.

## 2. PDF page identity vs text identity vs viewport identity

These are related but not interchangeable:

- native page index: PDF physical page;
- viewport-owned page: page currently dominant/visible in continuous scrolling;
- page-local canonical sentence index;
- document-global canonical sentence index;
- TTS bounded-window audio index.

Past bugs came from integer values crossing these boundaries silently.

Whenever a new PDF/TTS bug appears, write down the coordinate domain for every index before changing code.

## 3. PDF geometry/highlight/follow is not "just draw a rectangle"

Future Goal 0023-like work must preserve the already-accepted text identity while adding geometry as evidence/projection.

Risks:

- token/line ordering may not correspond cleanly to canonical sentence segmentation;
- multiple rectangles may represent one sentence;
- ligatures, columns, hidden text, duplicated text layers, or unusual reading order can make geometry unreliable;
- continuous viewport may show several pages at once;
- TTS follow must not fight explicit user scrolling;
- low-confidence geometry must degrade explicitly rather than fake precision.

The likely architecture is sentence -> native page -> page-relative rect set + confidence/provenance, projected into the continuous viewport.

## 4. Hostile/mixed/scanned PDF recovery

This is deliberately not current baseline work.

The old Quack-check-derived system contains useful ideas but also historical debt:

- Python subprocess lifecycle;
- Docling version drift;
- OCR dependencies;
- pypdf/pypdfium2;
- hardcoded/local Python environment assumptions;
- possible long-running whole-document work;
- uncertain geometry lineage.

Future recovery must sit behind a typed provider result with at least:

- source digest/identity;
- provider/version;
- page-aligned text;
- geometry if present;
- confidence/provenance;
- audit information;
- cancellation/stale-result behavior.

Recovery failure can never poison the visual PDF.

## 5. EPUB Pretty/Text-only projection safety

The A7 physical failure proved that view switching can damage presentation even when the underlying source is healthy if commands are relative and asynchronous.

A8 introduced target-state semantics. Future UI state changes should follow the same pattern.

If EPUB rendering ever appears truncated after a mode switch, investigate session/view-state races before blaming EPUB ingestion or the source file.

## 6. Reflow vs spoken-highlight follow

Goal 0014 solved violent reflow jumps, but a residual remains under severe cumulative metric edits.

The difficult part is precedence:

1. explicit user scrolling/dragging must win;
2. an active TTS follow request may need to win over idle reflow restoration;
3. otherwise the retained semantic reflow witness should preserve the reading neighborhood;
4. no permanent pinning to the highlight.

Goal 0015 owns stronger temporary viewport-band behavior for an already-visible spoken highlight.

## 7. Caliberate 105k-book scale

Anything that looks O(total catalog) deserves suspicion.

Known problematic pattern:

- warm startup repeatedly scanning ~104k rows to rediscover thumbnail files;
- several scans can hit multi-second budgets;
- giant catalog cache rewrites follow.

Goal 0017 should replace repeated rediscovery with lazy/indexed association and make provider pressure transient/backed off rather than producing per-frame retry/log storms.

The cover UX must also remain theme-readable; yellow-on-white transient errors were physically poor.

## 8. Windows QA environment accumulation

Repeated `qa.ps1` in the same PowerShell session can eventually fail in Visual Studio environment setup with:

`The input line is too long.`

Likely cause: repeated import of `VsDevCmd.bat` environment into the already-mutated parent process.

Until Goal 0018, use a fresh PowerShell for physical QA.

A future fix should make the bootstrap idempotent rather than merely documenting the workaround.

## 9. Windows TTS: ordinary path accepted, Natural/HD intentionally dormant

Ordinary Windows TTS works and is part of the accepted product.

Windows Natural/HD/Narrator-style voice capability is not currently authorized work.

Do not let a voice enumeration bug or feature request silently reopen Goal 0011.

## 10. TTS pause/resume and prefetch complexity

The runtime has multiple interacting concerns:

- backend-neutral synthesis;
- prefetch;
- first-sample boundaries;
- pause/resume behavior;
- page-local bounded normalization plans;
- canonical global sentence identity;
- user seek/repeat commands;
- persistence/bookmark updates.

A local fix in one of these can easily duplicate speech or advance the cursor early.

Tests should assert audible-boundary identity, not only control state.

## 11. Search has two asynchronous layers

Search UI and document-wide search are not the same state machine.

UI layer:

- panel open/closed;
- editor focus;
- immediate draft text.

Canonical layer:

- accepted query revision;
- document-wide match computation;
- selected match;
- native page provenance;
- stale source/generation rejection.

Conflating these layers caused both the disappearing panel and the A8 typing-race review failure.

## 12. PDF text cache vs source materialization vs raster cache

Three separate things:

- source PDF path/materialized file;
- trusted native text/cache artifact;
- in-memory raster textures.

Do not infer a missing cache feature because one of the other layers has no visible files.

## 13. Cold-start performance must be measured with cache state named

`-ResetQaState` destroys the isolated QA cache tree. A cold run after reset is not comparable to a normal warm reopen.

When reporting latency, always say whether the source/provider/catalog/materialization/text cache was cold or warm.

## 14. PDF zoom transition polish

Goal 0020 is accepted.

The remaining issue is specifically:

- Fit Width / Fit Page establishes an effective fit zoom;
- pressing +/- afterwards resumes from remembered prior manual zoom instead of stepping from the effective fit value.

Do not reopen continuous-scroll architecture for this; Goal 0021 is a narrow transition-polish problem.

## 15. H3 / heading-specific EPUB defects

An earlier concern existed around H3-specific rendering, but it was not verified against a book actually containing H3.

Do not promote speculative heading bugs without a reproducing source.

## 16. Physical QA workload must stay focused

The human maintainer is not a generalized test runner.

Director should reject source/CI defects before physical QA whenever possible.

Physical QA should target things machines cannot prove well:

- perceived responsiveness;
- audible speech;
- visual rendering completeness;
- real focus/keyboard interaction;
- continuous scrolling behavior;
- actual state continuity under abuse.

## 17. Repository/history preservation

The project is now complex enough that losing chat history would cause real architectural regression risk.

Therefore:

- every rejection should explain the defect and the principle learned;
- every physically accepted behavior should be written down;
- dead ends should remain searchable;
- current status should stay concise, while this archive preserves history;
- new substantial work should update durable history when it discovers a new invariant.
