# LanternLeaf Knowledge Archive

This document exists so LanternLeaf's hard-won engineering knowledge survives chat loss, browser failure, agent turnover, and context-window truncation.

It is deliberately broader than `current-status.md`. Current status answers **what is true now**. This archive answers **how we got here, what failed, what was learned, what must not be forgotten, and which unresolved problems still carry historical context**.

Treat this as a durable memory surface for the director, Codex, and future maintainers.

## Product identity

LanternLeaf is a local-first long-form reader whose central product claim is that **reading and listening are one synchronized activity**.

The authoritative conceptual chain is:

`source document -> canonical reading semantics -> visual projection + TTS projection -> first-sample playback identity -> reader cursor/bookmark/search state`

Formats are inputs and presentation surfaces, not separate products. EPUB, HTML, Markdown, TXT, PDF, and future Word-like formats should converge on shared reader/session semantics.

The project has repeatedly learned that visual state, TTS state, search state, and bookmark state must not independently invent identity.

## Current architectural center of gravity

The restarted desktop product is native Rust + `eframe`/`egui`.

Historical Tauri/React/WebView code is reference material, not the authority.

The migration away from web ownership was driven by real operational pain:

- hidden lifecycle layers;
- renderer stalls and lag;
- difficult ownership of playback and visual state;
- unreliable reasoning about asynchronous browser/UI behavior;
- poor fit for direct PDF/TTS synchronization.

The native direction is not purity for its own sake. It exists because the reader needs explicit, inspectable ownership.

## Core operational invariant

**The egui/render thread renders and commits small state transitions. It must not perform document-scale work, native PDF extraction, cache persistence, network calls, synthesis, or other heavy/blocking work.**

This rule has been rediscovered several times through regressions. Treat violations as architecture failures, not ordinary performance bugs.

## Project restart and governance

The modern restart adopted a repository-owned director/Codex workflow because chat-only iteration was losing architecture and forcing the human maintainer to act as a courier.

Roles:

- human principal: local Windows/runtime/GUI/audio testing and final product judgment;
- ChatGPT director/architect/integrator: architecture, scope, roadmap, goal contracts, review, integration, status;
- Codex: bounded implementation worker for one authorized repository macro-goal.

The repository is the durable communication channel. Important decisions, rejections, reports, acceptance evidence, and architecture boundaries must survive outside chat.

A **repository goal** is durable and may span many attempts. A **Codex Goal** is one bounded worker session. If a worker session terminalizes and the director rejects the attempt, the same repository goal continues in a NEW Codex Goal.

## Windows recovery: what the restart taught us

The restart began by making Windows a real product platform rather than a secondary compile target.

### Goal 0001 — Windows build recovery

The old project carried native build assumptions that made Windows recovery painful. Goal 0001 removed runtime bindgen/libclang requirements, hardened vendored native bindings, added Windows CI, and established a real Windows build baseline.

The lesson was structural: build-time native dependency discovery must not make ordinary end-user/developer startup depend on a fragile local toolchain accident.

The first goal was only partially sufficient. Build success did not imply trustworthy tests or trustworthy native launch, so later baseline work continued instead of declaring the platform solved too early.

### Goals 0002–0003 — baseline closure / native execution truth

The early restart phase separated "compiles" from "actually runs correctly on the target desktop". The project hardened the Windows-native execution path, repo-native setup, and CI evidence instead of accepting cross-platform assumptions as proof.

This established the policy that physical Windows behavior is authoritative when it contradicts synthetic confidence.

### Goal 0004 — Windows TTS backend

Ordinary Windows speech became a first-class backend rather than a workaround. The project converged on backend-neutral reader semantics so Windows TTS and Piper could share canonical playback ownership.

### Goal 0005 — TTS correctness / terminal goal notification

Two distinct lessons came out of this phase:

1. TTS correctness depends on **first-sample ownership**. A sentence becomes authoritative when the audible sample starts, not when synthesis is requested or queued.
2. The human should not poll Codex manually. Repository-owned goal completion signaling and Windows notifications were added so a pushed terminal attempt becomes inspectable before the human is interrupted.

### Goal 0006 — non-PDF reader parity

EPUB/HTML/Markdown/TXT were brought onto the native reader path with canonical sentence identity, text-only/pretty projections, search, navigation, and TTS parity.

### Goal 0007 — Windows QA bundle

The human workflow became repo-native: dependencies, isolated QA state, build, and launch are owned by repository scripts. Downloading CI artifacts is not an acceptable manual-development fallback.

## Caliberate integration: service boundaries and real-library scale

### Goal 0008 — first-class Caliberate reader integration

Caliberate became a provider rather than an external manual step. LanternLeaf learned to distinguish provider-unavailable failures from corrupt-book/materialization failures.

Important physical lesson: one troublesome Caliberate book (`42866`) was not corrupt; it failed because Caliberate was not running. The classification mattered because the wrong diagnosis would have sent work into materialization/corruption handling.

Accepted outcome:

- Caliberate materializes books for LanternLeaf;
- EPUB opens quickly;
- ordinary Windows TTS works;
- native pretty reading and spoken sentence ownership work together.

### Goal 0010 — catalog covers/provider UX

A major dead end was treating a book materialization path as a cover path. The accepted design added a narrow Caliberate `GET /api/v1/books/{id}/cover` contract so a thumbnail never requires materializing the EPUB.

LanternLeaf then added lazy visible/near-visible cover requests, bounded/coalesced concurrency, per-book ownership/freshness, and explicit provider-unavailable vs cover-unavailable states.

The real catalog constraint is enormous: roughly 105,570 books, with the provider list API capped at 500 per page. This forced progressive loading and made O(total-catalog) mistakes visible.

### Goal 0016 — starter library live-state continuity

The first approach was rejected because catalog reconciliation could overwrite live cover state, provider failures could be masked, and overlapping provider walks could occur.

The accepted design makes the starter usable while the provider is still loading, publishes batches progressively, preserves live per-book cover state through final reconciliation, and refreshes Recents in-process after successful source persistence.

A 5–7 second first open after `-ResetQaState` was correctly classified as **true cold cache behavior**, not a regression. Warm reopen remained immediate.

## TTS: the hardest non-PDF lessons

### Goal 0009 — playback polish and layered voices

Accepted physical behavior includes:

- sustained ordinary Windows speech;
- correct Play/Pause;
- no unsolicited duplicate ordinary lines;
- pretty and text-only spoken-sentence highlight/follow;
- Zira as portable ordinary Windows default;
- per-book voice persistence;
- transactional Piper rejection/recovery;
- Close Book and Safe Quit.

A recurring lesson is that playback, visual highlight, and session cursor must share one canonical identity. "Looks highlighted" is not enough if it is not the sentence whose audio actually started.

Windows Natural/HD voice work was deliberately deferred. Do not accidentally resurrect Goal 0011 while working on ordinary voices.

## Native pretty reader: performance and geometry lessons

### The critical lag incident

An early egui EPUB presentation path became catastrophically laggy: hover reactions could take seconds and the view was effectively unusable. This was especially important because egui had been chosen specifically to escape the latency experienced with the old web/Tauri presentation.

The fix dramatically improved responsiveness, and the lesson became permanent: native UI is not automatically fast. Heavy layout, rebuild, IO, image work, or document processing can still destroy responsiveness if placed on the interactive path.

### Goal 0012 — presentation controls and inline images

This goal fixed a long cluster of real presentation defects:

- hidden 720px content clamp producing giant gutters;
- vertical blank-scroll behavior;
- table/TOC compression;
- settings clipping;
- blockquote presentation;
- missing-font fallback;
- word/letter/line/paragraph/heading spacing;
- inline images and bounded media sizing;
- stale TTS highlight behavior under virtualization.

Physical QA, not screenshots alone, was required because several controls needed to be aggressively manipulated to prove they were wired to real geometry.

### Goal 0013 — starter panel containment

The starter became responsive using actual center width, a deterministic 1120px breakpoint, one-column fallback, bounded groups, wrapping, and long-string truncation.

### Goal 0014 — reflow anchor stability

Violent scroll jumps during presentation edits exposed a deep rule: you cannot repeatedly recapture the viewport anchor from unstable intermediate geometry.

The accepted solution keeps one semantic witness from the last stable geometry, carries it through the reflow transaction, waits for measured geometry, then restores once. Explicit user wheel/drag input and pending TTS follow have defined precedence.

Residual severe text-metric/highlight-band drift remains Goal 0015 and must not be confused with the already-solved violent-jump defect.

## PDF restart: the decisive architectural reset

The project originally contained old Quack-check-derived PDF transcription/recovery machinery. That system is now treated as **archaeology, not baseline infrastructure**.

Its useful ideas include explicit policy, deterministic identity, quality tiers, bounded chunking, backend separation, and audit artifacts.

Its dangerous historical assumptions include:

- Python subprocesses;
- pypdf/pypdfium2;
- Docling/OCR runtime dependencies;
- environment-specific Python discovery;
- a hardcoded Unix-like Docling path in `conf/quack-check.toml`;
- whole recovery/transcription concepts being too close to the basic act of opening a PDF.

The modern trust hierarchy is:

1. native visual Pdfium truth;
2. trustworthy native embedded text through the same Pdfium owner;
3. only later, typed hostile/mixed/scanned recovery behind a subordinate provider boundary.

Hard invariant:

**A broken text extractor may cost LanternLeaf TTS for that PDF. It may never cost LanternLeaf the PDF.**

## Goal 0019 — native PDF visual stability

This goal established the real PDF baseline:

- bundled/native Pdfium;
- one shared process-wide `PdfNativeService` / one Pdfium owner;
- metadata and rendering through the same owner;
- visual-first open independent of Quack-check/Python/Docling/OCR;
- explicit native PDF page domain;
- stale source/generation/page/size identity;
- deterministic raster residency with current page pinned;
- bounded PDF precheck;
- failure terminalization that cannot poison the visual reader.

Physical QA on a real 638-page Caliberate PDF proved the visual path works and can be abused without collapsing.

## Goal 0020 — continuous PDF viewport and zoom

The first attempt was rejected because the planner only knew the current page, Reader independently computed visibility, Fit Page was hard-coded, high zoom was effectively vertical-only, focal preservation was not semantic, aspect/layout information arrived too late, and geometry work could be O(total pages) per frame.

A later attempt was rejected because scheduler and presentation used mismatched width/key semantics, and zoom/fit state was absent from viewport commit identity, allowing stationary zoom to blank.

The accepted A3 design introduced:

- canonical `PdfRenderSpec` with source/generation/final raster dimensions;
- shared scheduler/presentation keys;
- render spec in replan identity;
- 25–400% zoom;
- Fit Width / Fit Page / Reset;
- horizontal + vertical scrolling;
- semantic viewport witness;
- cached geometry;
- continuous adjacent-page presentation.

Physical result: violent scrolling through the 638-page PDF was extremely responsive; page ownership kept up; adjacent pages appeared at seams; zoom controls worked.

Residual: after Fit Width/Fit Page, manual +/- resumes the old remembered manual ladder rather than stepping from the current effective fit percentage. This is Goal 0021, minor polish.

## Goal 0022 — native PDF embedded text/TTS: why it has taken so many attempts

Goal 0022 is intentionally strict because it establishes the canonical native PDF text path that later geometry/highlighting/recovery will depend on.

### A1 rejection

A1 extracted the whole PDF text as one uninterrupted Pdfium job. On a large document that could monopolize the only Pdfium owner and starve visible raster rendering.

It also performed document-scale sentence splitting/index construction and cache persistence on the egui thread.

Lesson: "same Pdfium owner" is correct, but background enrichment must be cooperative, and adopting text must be a bounded UI commit.

### A2 rejection

A2 introduced cooperative extraction and off-thread preparation, but trusted text adoption failed to promote the real production PDF capability policy. Text could exist internally while Text-only/search/TTS remained disabled.

A2 also still built a full document snapshot on the egui commit path, only prioritized `Current` raster requests rather than all queued visual work, and reopened/reparsed the PDF once per extracted page.

Lesson: capability state must become truthful atomically, and cooperative work must not amplify native document opens absurdly.

### A3/A4 lineage

The next attempts moved toward a prepared immutable native-text payload, bounded commits, policy promotion, visual priority, and reduced PDF-open amplification. They exposed the difficulty of preserving mutable live session state while swapping in document-scale immutable text state.

### A5 rejection

A5's shared trusted document was directionally correct, but canonical global sentence identity still fell back to stale pre-enrichment page counts in some paths. That could corrupt later-page TTS identity and final-document exhaustion.

Search also had a race where a query entered before enrichment survived as text but did not automatically gain matches when trusted text arrived.

Lesson: once an immutable trusted PDF document exists, **all global identity must use it**, and live UI state such as the current query must be reconciled against enrichment rather than frozen at worker start.

### A6 rejection

A6 fixed global identities and search reconciliation but used per-snapshot/per-session `std::thread::spawn()` to retire large shared PDF payloads off egui. Continuous scrolling could therefore create many short-lived OS threads.

Its "bounded" enriched snapshot also still performed O(total-pages) scans for some prefix computations.

Lesson: deferred destruction itself needs a bounded architecture, and bounded projections must be structurally independent of total document size.

### A7 director acceptance, then physical rejection

A7 introduced a single process-wide PDF retirement worker and prefix-indexed bounded projections. It passed source/CI review and reached real desktop QA.

Physical QA found:

- continuous PDF scrolling still excellent;
- transient `Rendering page N` placeholders disappear quickly;
- Text-only works and appears page-aligned;
- PDF visual <-> Text-only switching is sane;
- close/reopen is snappy;
- but natural PDF TTS page crossing was inconsistent;
- title/front matter could read once and stop;
- Next could occasionally repeat;
- Search panel appeared then immediately disappeared;
- rapid EPUB Pretty/Text-only toggling could destroy presentation, leaving only a fragment rendered.

This was the most valuable kind of failure: core native PDF visual work survived, while interaction/state bugs were exposed without invalidating the architecture.

### A8 rejection before another human QA pass

A8 added:

- explicit next-non-empty-page PDF TTS transition;
- persistent Search panel visibility;
- real Search `TextEdit`;
- idempotent `SetTextOnly { enabled }` semantics instead of relative toggles;
- an EPUB rapid-toggle stress regression.

Director source review still found three blockers:

1. Search shortcut suppression tracked the one-shot focus request rather than actual editor focus, so query characters could still trigger reader shortcuts after focus acquisition.
2. Search text was reconstructed every frame from asynchronously acknowledged reader state, so rapid typing could flicker/drop characters.
3. the PDF TTS continuation fix was only tested by directly calling the helper, not through the real TTS runtime loop; additionally, document-global canonical IDs were incorrectly compared with page-local normalization-plan bounds on later pages.

A9 is the active correction attempt at the time this archive was written.

## Search: durable lesson

Search has two distinct domains and must keep them separate:

- **UI draft/input state**: synchronous, frame-local/app-owned, must never lose keystrokes waiting for worker acknowledgement;
- **canonical document search state/results**: can be asynchronous/off-thread and stale-rejected.

The same distinction applies to actual keyboard focus vs a request to acquire focus.

## Text-only / Pretty: durable lesson

Relative asynchronous toggles are dangerous.

The failed EPUB stress behavior came from combining optimistic local state, stale asynchronous snapshots, and `ToggleTextOnly`. Reapplying a relative toggle can invert the user's desired state.

The accepted direction is idempotent desired-state semantics:

`SetTextOnly { enabled: bool }`

This pattern should be preferred anywhere an asynchronous acknowledgement could race with repeated UI intent.

## Caching lessons

Not every fast reopen implies a durable file must be visible where expected.

For PDFs:

- Caliberate may materialize the source PDF in LanternLeaf's cache hierarchy;
- direct filesystem PDFs need not be duplicated;
- native rendered page textures are intentionally bounded in memory and are not a durable forest of PNG/JPEG files;
- native trusted text cache is separate from raster residency.

`qa.ps1 -ResetQaState` intentionally deletes `.qa/windows`, including cache/materialized/derived QA artifacts. Use it only when deliberately testing cold state.

## Known unresolved / queued issues

- Goal 0015: severe reflow/highlight viewport-band polish.
- Goal 0017: cover backpressure and warm-cache thumbnail hydration scaling; avoid O(105k) rediscovery scans and giant rewrites.
- Goal 0018: repeated `qa.ps1` in one PowerShell can accumulate Visual Studio environment state until `VsDevCmd.bat` fails with `The input line is too long`; use a fresh PowerShell until fixed.
- Goal 0021: Fit Width/Fit Page -> manual +/- should step from current effective fit zoom.
- Goal 0022: native trustworthy PDF text/TTS/search baseline remains active.
- Next after Goal 0022 physical acceptance: native PDF sentence geometry/highlight/follow.
- Only after the native path is accepted: hostile/mixed/scanned recovery provider; Quack-check may be mined for ideas but cannot become a prerequisite.
- Goal 0011 Windows Natural/HD voices remains explicitly deferred.

## Rules future maintainers must not casually violate

- Visual PDF open must never depend on text extraction/recovery.
- There is one process-wide Pdfium owner.
- Heavy/document-scale work stays off egui.
- Raster/current visual responsiveness wins over background text enrichment.
- Exact visual synchronization must not be faked without real geometry.
- First audible sample owns canonical TTS progress.
- Search input draft and actual focus are UI state, not delayed worker state.
- Asynchronous UI mutations should prefer idempotent target-state commands over relative toggles.
- Physical Windows QA can invalidate synthetic confidence.
- A failed worker attempt is not a failed repository goal; preserve the lineage and lessons.
- The repository, not chat history, is the durable project memory.
