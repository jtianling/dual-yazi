## 1. Core Data Structures

- [x] 1.1 Create `UndoOp` enum and `UndoEntry` struct in `yazi-core/src/mgr/undo.rs` with variants: Rename, Create, Copy, Move
- [x] 1.2 Create `UndoManager` struct with undo/redo `Vec<UndoEntry>` stacks, `push()`, `undo()`, `redo()` methods, and max 20 entry limit
- [x] 1.3 Add `pub undo: UndoManager` field to `Mgr` struct and initialize in `Mgr::make()`

## 2. Parser and Proxy

- [x] 2.1 Add `UndoOpt` and `RedoOpt` parser types in `yazi-parser/src/mgr/`
- [x] 2.2 Add `MgrProxy::undo_push()` method and `UndoPushOpt` parser type for async undo recording

## 3. Undo/Redo Actors

- [x] 3.1 Create `Undo` Actor in `yazi-actor/src/mgr/undo.rs` — pops from undo stack, spawns tokio task to execute reverse operation via `provider::*`, emits `FilesOp` events for UI update, pushes entry to redo stack
- [x] 3.2 Create `Redo` Actor in `yazi-actor/src/mgr/redo.rs` — pops from redo stack, spawns tokio task to re-execute operation, emits `FilesOp` events, pushes entry to undo stack
- [x] 3.3 Register `on!(undo)` and `on!(redo)` in `yazi-fm/src/executor.rs` mgr section

## 4. Record Undo in Existing Actors

- [x] 4.1 Modify `Rename::do()` in `yazi-actor/src/mgr/rename.rs` — after successful `provider::rename`, record `UndoOp::Rename { old, new }` via MgrProxy
- [x] 4.2 Modify `Create::do()` in `yazi-actor/src/mgr/create.rs` — after successful create, record `UndoOp::Create { target, is_dir }` via MgrProxy
- [x] 4.3 Modify `Paste::act()` in `yazi-actor/src/mgr/paste.rs` — before dispatching, capture source→dest mapping and record `UndoOp::Move` (cut) or `UndoOp::Copy` (copy)
- [x] 4.4 Modify `CopyTo::act()` in `yazi-actor/src/mgr/copy_to.rs` — record `UndoOp::Copy { created }` with destination paths
- [x] 4.5 Modify `MoveTo::act()` in `yazi-actor/src/mgr/move_to.rs` — record `UndoOp::Move { pairs }` with source-dest pairs

## 5. Integration and Polish

- [x] 5.1 Ensure FilesOp events are correctly emitted in undo/redo actors so directory listings update
- [x] 5.2 Handle edge cases: undo when target file no longer exists (skip silently), undo on empty stack (no-op)
- [x] 5.3 Verify `u` keybinding works in normal mode without conflicting with `casefy lower` in visual mode
