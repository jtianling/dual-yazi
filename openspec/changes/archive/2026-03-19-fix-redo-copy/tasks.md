## 1. Data Model Change

- [x] 1.1 Change `UndoOp::Copy { created: Vec<UrlBuf> }` to `UndoOp::Copy { pairs: Vec<(UrlBuf, UrlBuf)> }` in `yazi-shared/src/undo_op.rs`

## 2. Update Copy Recording Sites

- [x] 2.1 Update `paste.rs` copy branch to build `(source, dest)` pairs from `mgr.yanked` and push `UndoOp::Copy { pairs }`
- [x] 2.2 Update `copy_to.rs` to build `(source, dest)` pairs and push `UndoOp::Copy { pairs }`

## 3. Update Undo Copy

- [x] 3.1 Update `undo.rs` `undo_copy` to extract dest from `pairs` (use `p.1`) for deletion instead of `created`

## 4. Implement Redo Copy via Scheduler

- [x] 4.1 In `redo.rs`, replace the skip comment with a call to `cx.core.tasks.scheduler.file_copy(from, to, true, false)` for each pair — this must happen in `act()` before the `tokio::spawn`, since it needs `&mut Ctx`
- [x] 4.2 Restructure `Redo::act()` to handle Copy in the synchronous `act()` path and other ops in `tokio::spawn`

## 5. Update Spec

- [x] 5.1 Update `openspec/specs/undo-redo/spec.md` to reflect Copy storing pairs and redo copy going through Scheduler
