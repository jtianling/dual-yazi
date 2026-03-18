## Why

The undo/redo system currently supports rename, create, copy, and move operations, but explicitly excludes trash (non-permanent delete). Trash is one of the most common file operations users perform, and accidental deletions are a frequent pain point. Since trashed files are recoverable by the OS, there is no technical reason to exclude them from the undo stack — the limitation was deferred due to implementation complexity around obtaining the trash destination path.

## What Changes

- Add a `Trash` variant to `UndoOp` that records `(original_path, trash_path)` pairs
- Modify `provider::trash()` to return the resulting trash path (capturing `resultingItemURL` on macOS, constructing the path on Linux)
- Thread the trash path through the scheduler pipeline via `FileOutTrash` and `HookInTrash`, injecting it during `reduce()`
- Add `push_trash_pair()` to `UndoManager` that merges consecutive trash results into a single undo entry
- Implement `undo_trash` (rename from trash_path back to original) and `redo_trash` (re-trash the file) in the undo/redo actors

## Capabilities

### New Capabilities
- `trash-undo`: Undo/redo support for trash (non-permanent delete) operations, including trash path capture, scheduler result threading, and restore logic

### Modified Capabilities
- `undo-redo`: Add Trash as a new undoable operation type alongside Rename, Create, Copy, and Move. Remove the requirement that trash operations are not recorded.

## Impact

- **yazi-fs**: `ProviderExt` trait — `trash()` return type changes from `io::Result<()>` to `io::Result<UrlBuf>`; macOS local provider captures `resultingItemURL`; Linux local provider constructs freedesktop trash path
- **yazi-vfs**: Transparent passthrough of new trash return type
- **yazi-scheduler**: `FileOutTrash::Succ` gains a `UrlBuf` payload; `HookInTrash` gains `trash_path` field; `FileOutTrash::reduce()` injects trash_path into the hook; `Hook::trash()` pushes undo via `MgrProxy`
- **yazi-shared**: `UndoOp` enum gains `Trash { pairs }` variant
- **yazi-core**: `UndoManager` gains `push_trash_pair()` method for merge-based recording
- **yazi-proxy**: `MgrProxy` gains `undo_push_trash_pair()` method
- **yazi-actor**: `Undo` and `Redo` actors gain trash-specific handlers
