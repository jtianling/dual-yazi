## Why

Redo copy is currently non-functional. `UndoOp::Copy` only stores destination paths (`created: Vec<UrlBuf>`), so when redo needs to re-copy files, it has no source paths and silently skips the operation. Additionally, redo copy should go through the Scheduler (like the original copy does) to get progress tracking, cancellation, and concurrency control for large directory copies.

## What Changes

- Change `UndoOp::Copy { created: Vec<UrlBuf> }` to `UndoOp::Copy { pairs: Vec<(UrlBuf, UrlBuf)> }` storing (source, dest) pairs — matching the `Move` variant's structure
- Update all sites that construct `UndoOp::Copy` (paste, copy_to) to capture source paths alongside destinations
- Update `undo_copy` to extract destinations from pairs for deletion
- Implement `redo_copy` to submit copy tasks through the Scheduler instead of skipping
- Update the undo-redo spec to reflect that Copy stores pairs, not just created paths

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `undo-redo`: Copy undo entries now store source-destination pairs instead of just destination paths; redo copy re-executes via Scheduler

## Impact

- `yazi-shared/src/undo_op.rs` — UndoOp::Copy variant signature changes
- `yazi-actor/src/mgr/paste.rs` — Build pairs instead of created list
- `yazi-actor/src/mgr/copy_to.rs` — Build pairs instead of created list
- `yazi-actor/src/mgr/undo.rs` — Extract dest from pairs for deletion
- `yazi-actor/src/mgr/redo.rs` — Implement redo_copy via Scheduler instead of skipping
- `openspec/specs/undo-redo/spec.md` — Update Copy scenarios to reference pairs
