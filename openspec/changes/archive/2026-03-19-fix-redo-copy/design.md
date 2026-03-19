## Context

The undo/redo system records file operations into an undo stack (max 20 entries) on `UndoManager`. Undo pops from the undo stack and pushes to redo; redo does the reverse.

Currently `UndoOp::Copy` stores only `created: Vec<UrlBuf>` (destination paths). This is sufficient for undo (delete the destinations), but redo cannot re-copy because it doesn't know the source paths. The redo actor currently skips copy operations entirely with a comment "Cannot redo a copy without original sources."

Normal (first-time) copy operations go through the Scheduler which provides progress tracking, cancellation, concurrency control, retry, and hook-based event emission. Undo/redo operations bypass the Scheduler and use raw `tokio::spawn`.

## Goals / Non-Goals

**Goals:**
- Make redo copy functional by storing source paths in `UndoOp::Copy`
- Route redo copy through the Scheduler for progress tracking and cancellation
- Keep the data model consistent: `Copy` and `Move` both store `(from, to)` pairs

**Non-Goals:**
- Routing other undo/redo operations (rename, create, undo-copy-delete) through the Scheduler — these are lightweight O(1) operations where Scheduler overhead is unnecessary
- Adding undo/redo support for trash, permanent delete, or plugin operations
- Preventing Scheduler hooks from firing during redo copy (acceptable side effect)

## Decisions

### Decision 1: Change `UndoOp::Copy` to store `pairs: Vec<(UrlBuf, UrlBuf)>`

**Choice**: Align Copy with Move — both store `Vec<(UrlBuf, UrlBuf)>` as (source, dest) pairs.

**Alternatives considered**:
- Add a separate `sources: Vec<UrlBuf>` field alongside `created` — rejected because it duplicates the Move pattern without good reason and two parallel vecs are fragile
- Store a `Yanked` + dest directory instead of expanded pairs — rejected because Yanked is tied to clipboard state that may change

**Rationale**: Pairs are self-contained, match the existing Move pattern, and contain everything needed for both undo (delete dest) and redo (copy source→dest).

### Decision 2: Redo copy goes through the Scheduler

**Choice**: In `Redo::act()`, when the op is `UndoOp::Copy`, call `cx.core.tasks.scheduler.file_copy()` for each pair directly from the `act()` method (which has `&mut Ctx`), instead of `tokio::spawn`.

**Alternatives considered**:
- `tokio::spawn` with raw `provider::copy` calls — rejected because large directory copies need progress tracking and cancellation, which only the Scheduler provides
- Build a `Yanked` and call `Tasks::file_copy()` — rejected because `Tasks::file_copy` takes `Yanked` + single dest dir, but redo pairs may have arbitrary dest paths

**Rationale**: The Scheduler's `file_copy(from, to, force, follow)` method accepts individual (from, to) pairs directly, which matches our data model. Using `force=true` since undo already deleted the destinations. Using `follow=false` as default.

### Decision 3: Redo copy still pushes entry back to undo stack synchronously

**Choice**: The redo entry is moved to the undo stack immediately (before the Scheduler finishes), same as how the original paste operation records optimistically.

**Rationale**: Consistent with the existing optimistic recording pattern. If the Scheduler copy fails, the undo entry becomes a no-op (same as today's behavior for all operations).

## Risks / Trade-offs

- **[Scheduler hooks fire on redo copy]** → Acceptable. Hook triggers `Pump::push_duplicate()` which is the same event as the original copy. This is correct behavior — other panes should know about the re-copied files.
- **[Source files may have been deleted between undo and redo]** → The Scheduler will report the error through its normal error handling. The redo entry still gets pushed to undo stack (optimistic), becoming a no-op on next undo. This matches existing behavior for all operations.
- **[Memory increase from storing source paths]** → Negligible. Max 20 undo entries × a few UrlBufs each.
