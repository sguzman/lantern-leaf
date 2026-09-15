# LanternLeaf Accepted Invariants

This file is the compact set of rules that repeated failures have promoted from preferences into invariants.

Changing one of these requires an explicit architecture decision, not incidental implementation convenience.

## Product invariants

### Reading and listening share one canonical identity

Visual presentation, TTS, bookmarks, search navigation, and persisted reading position must project one canonical document identity.

### First sample owns spoken progress

The sentence whose audible sample begins owns the spoken cursor. Synthesis request, queue insertion, or speculative prefetch does not.

### Formats converge on semantics

EPUB, HTML, Markdown, TXT, PDF, and future formats should map into common reader/session semantics instead of inventing incompatible playback models.

## UI / threading invariants

### egui does bounded interactive work only

No document-scale parsing, indexing, native PDF extraction, network work, synthesis, cache persistence, or unbounded destruction may occur on the egui/render thread.

### Native UI is not presumed fast

Responsiveness is proven by bounded architecture and physical behavior, not by the fact that egui is native.

### UI draft state and canonical async state are different things

Text entry, focus ownership, and immediate desired state belong to synchronous UI/app state.

Document-wide search results, persistence, source loading, and background enrichment may be asynchronous.

Do not rebuild live text input from delayed worker acknowledgement every frame.

### Prefer target-state commands over relative async toggles

For state that can race with delayed snapshots, prefer commands like:

`SetTextOnly { enabled: bool }`

rather than `ToggleTextOnly`.

Same-target requests should be harmless/no-op and stale acknowledgements must not invert current intent.

## TTS invariants

### Playback cursor identity is canonical and global

Page-local normalization indexes may not masquerade as document-global sentence IDs.

### Coordinate domains must remain explicit

- document-global canonical sentence ID: document identity;
- native page index: PDF page identity;
- page-local sentence index: local page ownership;
- bounded normalization-plan index: local audio preparation membership.

Never compare values from different coordinate domains merely because they are all integers.

### Natural continuation must move canonical session state

At a real native PDF page boundary, TTS continuation advances ReaderSession to the next non-empty native page before rebuilding the local plan.

### Final exhaustion is terminal

The last sentence of the document must not be reconstructed as the beginning of another bounded playback window.

## PDF invariants

### Visual PDF truth is tier 0

A PDF with broken/missing text must still open visually if Pdfium can render it.

### Text extraction may fail without losing the PDF

**A broken text extractor may cost LanternLeaf TTS for that PDF. It may never cost LanternLeaf the PDF.**

### One process-wide Pdfium owner

Metadata, raster work, and native embedded-text extraction share one process-wide native PDF service/Pdfium owner.

Do not introduce a second owner simply to obtain concurrency.

### Visual raster work outranks background text enrichment

Current and Nearby raster work must be serviced before another background text unit when queued.

Visible page responsiveness is authoritative.

### Native text enrichment is cooperative

Large PDFs must not be monopolized by one whole-document native extraction call.

Yield between bounded units/chunks while materially reducing document-open amplification.

### Trusted PDF text remains native-page aligned

Canonical relationship:

`native PDF page -> accepted page text -> canonical sentences on that native page`

Do not line-count-repaginate a trusted native PDF transcript into synthetic pages.

### Shared trusted PDF document is authoritative for global identity

Once trusted text is adopted, global sentence IDs, page prefix sums, search provenance, bookmarks, before/after checks, and TTS continuation must derive from the trusted prepared document.

### Bounded PDF projection is independent of document size

Current-page projection can clone current-page-local data and Arc-clone shared immutable document state.

It may not flatten/scan/clone the entire PDF on every update.

### Large shared PDF payload destruction stays off egui and bounded

Use the process-wide retirement worker/queue. Never spawn one OS thread per snapshot merely to drop Arcs.

### Raster cache is ephemeral

Do not turn native PDF rendering into a durable page-image cache unless a separate architecture decision explicitly requires it.

### Do not fake exact PDF geometry

Trusted embedded text without sentence rectangles enables Text-only/search/TTS, not exact pretty-page sentence highlight.

`pretty_sync_enabled` / exact sentence sync must remain false until geometry is actually available.

## Quack-check / recovery invariants

### Quack-check is not baseline infrastructure

The old recovery stack is source material/archaeology.

Goal 0022 and the native trustworthy path must not depend on:

- Quack-check subprocess orchestration;
- Python;
- Docling;
- OCR;
- pypdf/pypdfium2 runtime assumptions;
- developer-specific Python paths.

### Hostile recovery comes later and behind a typed boundary

Mixed/scanned/hostile recovery may reuse or rewrite older components only as optional background providers after the native visual + trustworthy embedded-text baseline is accepted.

## EPUB / pretty invariants

### Pretty document completeness must survive view switching

Text-only/Pretty switching is a projection change, not a destructive document transformation.

Rapid alternating requests must converge to the final requested view without truncating the structured EPUB document.

### Reflow anchoring uses last-stable geometry

Do not recapture a semantic viewport witness from unstable intermediate reflow frames.

### Explicit user scrolling beats automatic correction

Pending reflow/TTS follow logic must yield to explicit user wheel/drag behavior according to the accepted precedence rules.

## Caliberate invariants

### Provider unavailable is a distinct class

Do not diagnose a stopped/unreachable Caliberate service as source corruption.

### Covers do not require book materialization

Use the narrow cover provider contract.

### Catalog work must scale below O(total library) per ordinary frame/warm action

The real catalog is ~105k books. Repeated full walks are product bugs even if individually "simple".

## QA / evidence invariants

### Real desktop behavior can override synthetic confidence

CI and deterministic tests are necessary, not sufficient, for visual/audio/interaction acceptance.

### Tests must exercise the production control flow they claim to prove

A helper unit test does not prove a runtime loop invokes that helper correctly.

### Cold and warm behavior are separate evidence classes

Do not call expected cold cache/materialization cost a regression simply because warm reopen is much faster.

### `qa.ps1 -ResetQaState` is destructive by design

It removes isolated QA cache/materialized/derived state. Use only when intentionally testing cold behavior.

### Fresh PowerShell remains the safe QA launcher until Goal 0018

Repeated Visual Studio environment import can cause `VsDevCmd.bat` environment growth and `The input line is too long`.

## Workflow invariants

### Repository is durable memory

Chat is coordination, not the sole archive.

Architecture, rejections, acceptance evidence, unresolved risks, and hard-won lessons belong in Git.

### Repository goal != Codex Goal

A repository goal survives multiple Codex Goal attempts.

If an attempt terminalizes and is rejected, use a new Codex Goal while preserving the same repository goal lineage.

### Human is not the courier

Codex pushes its branch/report; director reviews GitHub directly. Human performs local physical QA only when explicitly authorized.

### No silent scope expansion

Codex owns the authorized macro-goal, not adjacent cleanup or architecture invention.

### Deferred work stays deferred

Windows Natural/HD voices (Goal 0011) remain dormant until explicitly re-authorized.