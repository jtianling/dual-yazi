## 1. 扩展 UndoOp 数据结构

- [x] 1.1 在 `yazi-shared/src/undo_op.rs` 中为 `UndoOp::Copy` 和 `UndoOp::Move` 添加 `overwritten: Vec<(UrlBuf, UrlBuf)>` 字段
- [x] 1.2 修复所有因新字段导致的编译错误(undo.rs, redo.rs 等文件中的模式匹配)

## 2. UndoManager 增加增量 pair 方法

- [x] 2.1 在 `yazi-core/src/mgr/undo.rs` 中添加 `push_copy_pair(from, to)` 方法, 追加 pair 到最新 Copy entry
- [x] 2.2 添加 `push_move_pair(from, to)` 方法, 追加 pair 到最新 Move entry
- [x] 2.3 添加 `push_copy_overwritten(original, trash_path)` 方法, 追加到最新 Copy entry 的 overwritten 字段
- [x] 2.4 添加 `push_move_overwritten(original, trash_path)` 方法, 追加到最新 Move entry 的 overwritten 字段

## 3. MgrProxy 增加代理方法

- [x] 3.1 在 `yazi-proxy/src/mgr.rs` 中添加 `undo_push_copy_pair`, `undo_push_move_pair`, `undo_push_copy_overwritten`, `undo_push_move_overwritten` 代理方法
- [x] 3.2 在 mgr actor 的消息处理中注册对应的 relay handler

## 4. 修改 paste actor 推送空 pairs

- [x] 4.1 修改 `yazi-actor/src/mgr/paste.rs`, copy 模式推送 `UndoOp::Copy { pairs: vec![], overwritten: vec![] }`
- [x] 4.2 修改 `yazi-actor/src/mgr/paste.rs`, cut 模式推送 `UndoOp::Move { pairs: vec![], overwritten: vec![] }`
- [x] 4.3 修改 `yazi-actor/src/mgr/copy_to.rs`, 推送空 pairs 的 Copy entry
- [x] 4.4 修改 `yazi-actor/src/mgr/move_to.rs`, 推送空 pairs 的 Move entry

## 5. Hook 回报实际路径

- [x] 5.1 修改 `yazi-scheduler/src/hook/hook.rs` 中的 `copy()` 方法, 完成时调用 `MgrProxy::undo_push_copy_pair(from, to)`
- [x] 5.2 修改 `yazi-scheduler/src/hook/hook.rs` 中的 `cut()` 方法, 完成时调用 `MgrProxy::undo_push_move_pair(from, to)`

## 6. 强制覆盖时 trash 原文件

- [x] 6.1 修改 `yazi-scheduler/src/file/file.rs` 中的 `copy()` 方法, 当 `force=true` 且目标存在时, 先 `provider::trash()` 原文件
- [x] 6.2 扩展 `HookInOutCopy` 增加 `overwritten: Option<(UrlBuf, UrlBuf)>` 字段, 传递 trash 信息
- [x] 6.3 修改 `hook.rs::copy()`, 将 overwritten 信息回报至 `MgrProxy::undo_push_copy_overwritten()`
- [x] 6.4 修改 `yazi-scheduler/src/file/file.rs` 中的 `cut()` 方法, 同样处理 force 覆盖的 trash
- [x] 6.5 扩展 `HookInOutCut` 增加 `overwritten: Option<(UrlBuf, UrlBuf)>` 字段
- [x] 6.6 修改 `hook.rs::cut()`, 将 overwritten 信息回报至 `MgrProxy::undo_push_move_overwritten()`

## 7. 更新 undo 执行逻辑

- [x] 7.1 修改 `yazi-actor/src/mgr/undo.rs` 中的 `undo_copy()`, 删除 pairs 中的目标文件后, 恢复 overwritten 中的原文件
- [x] 7.2 修改 `undo_move()`, 移回 pairs 中的文件后, 恢复 overwritten 中的原文件

## 8. 更新 redo 执行逻辑

- [x] 8.1 修改 `yazi-actor/src/mgr/redo.rs` 中 Copy 的 redo, 确保 overwritten 字段在 redo 时被正确传递(redo copy 使用 force=true, 不需要额外处理 overwritten)
- [x] 8.2 修改 Move 的 redo, 确保 overwritten 字段兼容
