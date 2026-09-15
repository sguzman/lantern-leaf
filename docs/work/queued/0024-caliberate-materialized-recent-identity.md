# Goal 0024 — Caliberate materialized source identity in Recents

## Status

**QUEUED — REAL-DESKTOP UX DEFECT**

## Physical evidence

A real Caliberate PDF materialized under LanternLeaf's QA cache appears in Recents as a hash-like filename such as:

`725-13e8b7a0`

with a path like:

`.../calibre-downloads/caliberate/725-13e8b7a0.pdf`

and no useful cover/title identity.

The materialized source cache itself is expected. The identity loss is not.

## Source diagnosis

Current Recents are reconstructed from the cached source path. `list_recent_books()` calls `infer_recent_title(&source_path)`, which falls back to the source file stem. For provider-materialized Caliberate files, that stem is an implementation identity, not a human title.

Thumbnail inference is also independent of the Caliberate catalog/provider record, so the recent row can lose the known cover even though the catalog has it.

## Required direction

Preserve provider provenance when a Caliberate book is materialized/opened.

A small durable provenance/identity sidecar or equivalent cache metadata should retain at least:

- provider = Caliberate;
- provider book ID;
- human title;
- authors when available;
- source format;
- cover/thumbnail identity or a safe way to resolve the existing cached/provider cover;
- materialized source path;
- enough versioning to reject stale provenance.

Recents should prefer durable provider identity over hash-like materialized filenames.

Requirements:

- title is human-readable after restart;
- cover is shown when the provider/catalog says one exists and a safe thumbnail can be resolved;
- do not materialize the book again merely to display a recent row;
- do not perform O(total-catalog) lookup on every Recent row;
- provider unavailable must not make an already-known recent title disappear;
- Delete Recent must clean associated provenance safely without deleting unrelated catalog cache;
- ordinary direct-filesystem sources continue to use filename/inferred metadata fallback;
- browser-tab recents retain their existing manifest identity.

This goal is deliberately separate from Goal 0022 PDF text/TTS correctness.
