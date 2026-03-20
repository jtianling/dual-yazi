## Why

Copy undo 在文件名冲突场景下删除了错误的文件. 当目标目录已存在同名文件时, 非强制粘贴会创建重命名后的副本(如 `file_1.txt`), 但 undo 记录的是原始文件名, 导致 undo 删除了原有文件而非新复制的文件.  此外, 强制覆盖粘贴(`P`)会销毁原文件, 且无法撤销.

## What Changes

- 修复 copy/cut undo entry 中记录的目标路径, 使其反映调度器实际使用的路径(经 `unique_file` 重命名后的路径)
- 建立从 scheduler 到 undo manager 的回调机制, 在 copy/cut 完成后更新 undo entry 中的 pairs
- 强制覆盖粘贴(`P`/`--force`)在覆盖前将原文件 trash, undo 时从 trash 恢复原文件并删除新复制的文件

## Capabilities

### New Capabilities

- `copy-undo-conflict`: 处理 copy/cut undo 在文件名冲突和强制覆盖场景下的正确行为

### Modified Capabilities

- `undo-redo`: 扩展 Copy/Move undo entry 以支持实际目标路径更新和强制覆盖时的原文件恢复

## Impact

- `yazi-actor/src/mgr/paste.rs` - undo entry 记录逻辑
- `yazi-actor/src/mgr/copy_to.rs` - 同上
- `yazi-actor/src/mgr/move_to.rs` - 同上
- `yazi-actor/src/mgr/undo.rs` - undo 执行逻辑, 需处理强制覆盖的恢复
- `yazi-shared/src/undo_op.rs` - UndoOp 数据结构可能需要扩展
- `yazi-core/src/mgr/undo.rs` - UndoManager 可能需要新的更新方法
- `yazi-scheduler/src/file/file.rs` - copy/cut 完成后回调
- `yazi-scheduler/src/hook/hook.rs` - Hook 机制扩展
