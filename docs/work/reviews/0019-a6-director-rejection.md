# Goal 0019 A6 — director rejection

## Decision

**REJECTED BEFORE REAL-DESKTOP QA — REOPEN AS A7**

A6 fixes the major ownership error from A5 in principle: the app now shares one `PdfNativeService`/`NativePdfRenderer` owner between source-open metadata and raster presentation, and detached effect execution has a panic boundary. Hosted Windows CI also passes the new shared-service probe.

Director source review found a blocking wakeup/liveness defect in the production service loop, so another human recheck would likely reproduce the same permanent `SourceLoading` symptom.

## Blocking defect — idle metadata requests are never serviced

`PdfNativeService` checks `metadata_rx.try_recv()` only at the top of the outer worker loop. If no metadata request is already waiting, it then enters an inner scheduler-wait loop.

That inner loop wakes every 10 ms (and can also be notified by `metadata()`), but it only checks shutdown and `scheduler.take_next()`. When there is no raster request, it immediately waits again. It never breaks back to the outer loop merely because the timeout fired or a metadata request arrived.

Therefore the common production sequence is broken:

1. app starts the native PDF service;
2. service becomes idle with no raster work and enters the inner scheduler wait;
3. later the user opens a PDF;
4. source-open enqueues a metadata request and notifies the condvar;
5. worker wakes, finds no raster key, and waits again without checking `metadata_rx`;
6. source-open blocks forever in `reply_rx.recv()` and the shell remains in `SourceLoading`.

This is the same visible failure class the real-desktop A5 run exposed, even though the underlying duplicate-Pdfium-owner bug is now removed.

## Why CI did not catch it

The dedicated lifecycle test calls `PdfNativeService::start()` and immediately calls `service.metadata(...)`. That timing allows the metadata request to race into the channel before the worker has settled into its idle scheduler-wait loop. The test therefore proves shared ownership, but not the production lifetime `start -> become idle -> metadata request`.

The report claims the 10 ms wait timeout prevents lost metadata wakeups, but the timeout does not return control to the outer metadata check; it stays inside the raster scheduler loop.

## A7 required correction

1. Replace the split metadata-channel + raster-condvar idle loop with a single liveness-safe request arbitration mechanism, or otherwise ensure every metadata wake/timeout returns to metadata polling before sleeping again.
2. Metadata requests for current source open must be serviced promptly when the native service is already idle, with no raster request required to kick the worker.
3. Preserve the single authoritative Pdfium owner and serialization of all native metadata/raster work.
4. Preserve off-egui execution, visual-first source open, PDF page-domain ownership, real zoom, current-priority raster scheduling, stale rejection, and deterministic residency.
5. Preserve detached effect panic terminalization.
6. Add a deterministic regression that starts the production native service, deliberately waits long enough for it to become idle, then requests metadata for a real multi-page PDF and requires bounded completion before requesting/rastering page 1.
7. Add a repeated idle-cycle test: metadata -> idle -> metadata again, proving no lost wakeup after the first successful operation.
8. Keep malformed input terminal and source switching safe.

## Secondary design note

`validate_pdf_source()` currently reads the entire PDF into memory merely to check `%PDF-` and `%%EOF`. This work is off the UI thread, so it is not the blocker, but A7 should make this precheck bounded (header plus bounded tail) or rely on the native parse path rather than imposing an avoidable full-file read before Pdfium opens large PDFs.

## Validation expectation

Do not request human QA until the idle-service regression passes in the production topology and the hosted Windows `native-workspace` + `hosted-renderer-probe` gates are green.

Goal 0019 remains open.