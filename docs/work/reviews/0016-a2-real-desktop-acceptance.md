# Goal 0016 A2 — real-desktop acceptance

## Decision

**ACCEPTED — GOAL 0016 CLOSED**

The focused Windows pass verifies the core Goal 0016 outcomes on the real large Caliberate library.

## Physical evidence

- From a reset QA cache, catalog rows became visible while the provider walk was still in progress rather than waiting for the full ~105k catalog.
- The starter shell displayed live progress such as `Loading catalog… 71738 / 105570`, proving progressive publication is visible and useful.
- The catalog completed and durable catalog/cache state survived restart.
- Opening a representative EPUB caused the recent entry to appear in the same running LanternLeaf process; restarting without `-ResetQaState` preserved that recent entry.
- Representative EPUB TTS and visual settings remained functional.
- Warm cached reopen after restart was immediate.

## Cold-open interpretation

The first EPUB open after `qa.ps1 -ResetQaState` took materially longer than the later warm reopen. This is not accepted as evidence of a Goal 0016 regression because the QA reset deliberately deletes `.qa/windows`, and `LANTERNLEAF_CACHE_DIR` points inside that tree. Caliberate materializations and LanternLeaf document artifacts therefore begin cold. The immediate warm reopen demonstrates the existing cache path is intact. Cold-open latency remains eligible for later measured performance work under general ergonomics/latency priorities.

## Residuals discovered during this pass

Two separate minor follow-ups were exposed but do not keep Goal 0016 open:

1. During the long progressive catalog walk, visible cover requests could transiently show `Cover fetch/decode failed` and repeatedly retry; once the catalog walk completed, covers successfully populated and cached. Source inspection shows the cover HTTP attempt is capped by the existing short thumbnail timeout and generic failures are immediately eligible for another visible-row request. This is queued separately as Goal 0017 so normal provider contention does not present as repeated scary failures or request churn.
2. A provider-down test correctly produced a catalog failure, but the starter rendered its error with literal bright yellow text on the light theme, making the message difficult to read. This readability defect is also owned by Goal 0017.

The repeated `VsDevCmd` `The input line is too long` failure in a reused PowerShell process is a QA-bootstrap idempotence defect rather than product behavior. It is queued separately as Goal 0018.

## Presentation-reflow residual

The physical pass still observed some presentation jerk under media-related settings and possibly heading-3 scaling. This does not reopen Goal 0014. Goal 0015 remains the dedicated queued viewport/reflow-polish item; heading-specific behavior should only be promoted if reproduced on a document that actually contains the affected heading level.

## Closure

Goal 0016 is closed. The next substantive product gate is native PDF visual stability. Goals 0015, 0017, and 0018 remain queued polish/engineering cleanup unless new evidence makes one materially disruptive enough to preempt PDF work.
