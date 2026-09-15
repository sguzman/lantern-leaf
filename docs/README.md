# LanternLeaf Documentation Map

LanternLeaf's repository is the durable project memory. Chat is coordination; Git is authority.

This index separates **current truth**, **durable history**, **architecture**, **work-state**, and **physical evidence** so future agents do not have to reconstruct the project from conversation history.

## Start here

- [`project/current-status.md`](project/current-status.md) — concise verified current state.
- [`project/priorities.md`](project/priorities.md) — current ordered priorities.
- [`project/philosophy.md`](project/philosophy.md) — product philosophy and semantic intent.
- [`project/product-scope.md`](project/product-scope.md) — product boundaries.
- [`project/roles-and-workflow.md`](project/roles-and-workflow.md) — human/director/Codex contract.
- [`project/windows-development.md`](project/windows-development.md) — Windows development/QA workflow.

## Durable project memory

These files intentionally retain history that would otherwise disappear when current-status documents are cleaned up.

- [`project/knowledge-archive.md`](project/knowledge-archive.md) — narrative history of the restart, major successful goals, rejected attempts, and the reasoning behind the current architecture.
- [`project/failure-and-dead-end-catalog.md`](project/failure-and-dead-end-catalog.md) — searchable catalog of failed approaches, misleading diagnoses, and corrections.
- [`project/accepted-invariants.md`](project/accepted-invariants.md) — compact rules promoted from repeated failures into architecture invariants.
- [`project/thorny-issues-and-open-risks.md`](project/thorny-issues-and-open-risks.md) — subsystem-boundary problems that are easy to misdiagnose.
- [`project/qa-evidence-ledger.md`](project/qa-evidence-ledger.md) — physical Windows/GUI/audio evidence that materially changed project truth.

## Architecture

See [`architecture/`](architecture/) for subsystem contracts and decision records.

Particularly important current PDF documents include:

- `architecture/pdf-renderer-contract.md`
- `architecture/pdf-text-recovery-boundary-2026-09.md`

Architecture documents define boundaries and invariants. Work-goal documents define bounded implementation outcomes.

## Roadmaps

See [`roadmaps/`](roadmaps/) for longer-horizon sequencing.

The restart master roadmap is the durable gate sequence; `project/current-status.md` remains the shorter source of what is actually accepted today.

## Work system

See [`work/README.md`](work/README.md).

Work moves through:

```text
queued -> ready -> active -> done/blocked -> director review
```

A repository goal may cycle back to `ready` after a rejected Codex attempt. The repository goal ID remains the same across correction attempts.

Important directories:

- [`work/queued/`](work/queued/) — known future work, not authorized yet.
- [`work/ready/`](work/ready/) — the single authorized next repository macro-goal.
- `work/active/` — current worker-owned execution when present.
- [`work/done/`](work/done/) — terminal attempt contracts/history.
- [`work/reports/`](work/reports/) — Codex implementation/validation evidence.
- [`work/reviews/`](work/reviews/) — director acceptance/rejection reasoning.

## Historical migration documents

Top-level migration/parity/Tauri documents remain useful as historical evidence of the transition from the older web/Tauri product toward the native Rust/egui authority.

They are not the current architecture unless a current project/architecture document explicitly references them.

## Documentation maintenance rule

When substantial work discovers a new failure mode, invariant, physical truth, or architectural dead end, update the durable memory layer rather than leaving that knowledge only in chat.

A good rule:

- **current-status** says what is true now;
- **goal/report/review** says what happened in one work lineage;
- **knowledge archive** says what the project learned;
- **failure catalog** says what not to rediscover;
- **accepted invariants** says what must not casually regress;
- **QA ledger** says what the real desktop actually proved.
