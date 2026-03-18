## 1. Provider Layer — trash() returns trash path

- [x] 1.1 Change `ProviderExt` trait: `trash()` return type from `io::Result<()>` to `io::Result<UrlBuf>` in `yazi-fs/src/provider/traits.rs`
- [x] 1.2 macOS local provider: replace `trash` crate call with direct `NSFileManager.trashItemAtURL:resultingItemURL:error:` using `objc2_foundation`, capture and return the `resultingItemURL` as `UrlBuf` in `yazi-fs/src/provider/local/local.rs`
- [x] 1.3 Linux local provider: after `trash::delete()`, locate the resulting file in the Freedesktop trash `files/` directory and return the path as `UrlBuf`
- [x] 1.4 SFTP provider: update `trash()` to return `io::Result<UrlBuf>` (still returns `Unsupported` error) in `yazi-vfs/src/provider/sftp/sftp.rs`
- [x] 1.5 Update `yazi-vfs/src/provider/providers.rs` and `yazi-vfs/src/provider/provider.rs` to propagate the new `UrlBuf` return type

## 2. UndoOp and UndoManager

- [x] 2.1 Add `Trash { pairs: Vec<(UrlBuf, UrlBuf)> }` variant to `UndoOp` enum in `yazi-shared/src/undo_op.rs`
- [x] 2.2 Add `push_trash_pair(original: UrlBuf, trash_path: UrlBuf)` method to `UndoManager` in `yazi-core/src/mgr/undo.rs` with merge-into-top-entry logic

## 3. Scheduler Pipeline — thread trash path to Hook

- [x] 3.1 Change `FileOutTrash::Succ` to `Succ(UrlBuf)` in `yazi-scheduler/src/file/out.rs`; update `reduce()` to inject `trash_path` into `task.hook` (HookInTrash)
- [x] 3.2 Add `trash_path: Option<UrlBuf>` field to `HookInTrash` in `yazi-scheduler/src/hook/in.rs`
- [x] 3.3 Update `File::trash_do()` in `yazi-scheduler/src/file/file.rs` to pass the returned `UrlBuf` into `FileOutTrash::Succ(path)`
- [x] 3.4 Update `Hook::trash()` in `yazi-scheduler/src/hook/hook.rs` to call `MgrProxy::undo_push_trash_pair(target, trash_path)` when `intact && trash_path.is_some()`

## 4. Proxy Layer

- [x] 4.1 Add `undo_push_trash_pair(original: UrlBuf, trash_path: UrlBuf)` to `MgrProxy` in `yazi-proxy/src/mgr.rs`
- [x] 4.2 Add corresponding `UndoPushTrashPair` actor that calls `cx.mgr.undo.push_trash_pair()` in `yazi-actor/src/mgr/`

## 5. Undo/Redo Actors

- [x] 5.1 Add `undo_trash(pairs)` handler in `yazi-actor/src/mgr/undo.rs`: rename each file from `trash_path` back to `original`, emit `FilesOp::Deleting` and `FilesOp::Upserting`
- [x] 5.2 Add `redo_trash(pairs)` handler in `yazi-actor/src/mgr/redo.rs`: call `provider::trash()` for each `original`, collect new trash paths, push updated `UndoOp::Trash` entry via `MgrProxy::undo_push()`
- [x] 5.3 Wire up `UndoOp::Trash` match arms in `Undo::act()` and `Redo::act()`
