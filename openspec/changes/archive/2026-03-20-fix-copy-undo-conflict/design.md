## Context

当用户执行非强制粘贴(p)且目标目录存在同名文件时, 调度器通过 `unique_file()` 自动重命名目标(如 `a.txt` → `a_1.txt`). 但 undo entry 在 `paste.rs` 中创建时使用的是原始文件名, 导致 undo 删除了原有文件而非新复制的文件.

当用户执行强制粘贴(P)时, 原文件被直接覆盖, 无法恢复.

现有的 trash-undo 机制提供了从调度器回报路径信息的范式: hook 完成时调用 `MgrProxy::undo_push_trash_pair()` 将实际路径注入 undo entry. 本修复复用此模式.

## Goals / Non-Goals

**Goals:**

- Copy/Move undo entry 记录调度器实际使用的目标路径(经 `unique_file` 重命名后的路径)
- 强制覆盖粘贴(P)前将原文件 trash, undo 时从 trash 恢复
- 复用现有 hook → MgrProxy → UndoManager 的回调模式, 保持架构一致性
- 同时修复 paste(p/P), copy_to(F5), move_to(F6) 三个入口

**Non-Goals:**

- 不处理 redo 中 overwritten 文件的重新 trash(redo copy 已使用 force=true, 直接覆盖即可)
- 不解决快速连续粘贴时 hook 回报到错误 undo entry 的竞态(与 trash 存在相同限制, 实践中极少发生)

## Decisions

### Decision 1: 扩展 UndoOp 数据结构

在 `UndoOp::Copy` 和 `UndoOp::Move` 中增加 `overwritten: Vec<(UrlBuf, UrlBuf)>` 字段, 存储 `(原始目标路径, trash_path)` 对.

**Why**: 强制覆盖场景需要记录被覆盖文件的 trash 位置以便恢复. 与 `UndoOp::Trash` 的 pairs 结构保持一致.

**Alternative**: 创建新的 `UndoOp::ForceCopy` 变体. 但这会导致 undo/redo 逻辑分支过多, 不如在现有变体上扩展.

### Decision 2: Hook 回报实际目标路径, 替代 paste 时预计算

`paste.rs` 推送空 pairs 的 Copy/Move entry 作为占位符. 调度器 hook 完成时回报实际 `(from, to)` 对, 注入到最新的 Copy/Move entry.

**Why**: 调度器才知道经 `unique_file()` 重命名后的真实路径. 此模式与 trash 完全一致: trash hook 回报 `(original, trash_path)` 对.

**Alternative**: paste 时预计算路径, hook 完成后更新. 但需要匹配逻辑, 且 `unique_file()` 是异步操作, 在同步的 paste actor 中无法调用.

### Decision 3: 强制覆盖时在 scheduler 层 trash 原文件

在 `file.rs::copy()` 和 `cut()` 中, 当 `force=true` 且目标已存在时, 先调用 `provider::trash()` 将原文件移至回收站, 然后正常执行覆盖. Trash 路径通过 hook 回报至 undo manager 的 `overwritten` 字段.

**Why**: scheduler 层是判断 force 和目标是否存在的唯一位置. 复用 `provider::trash()` 避免自建临时存储. 回收站有平台原生支持, 用户也能在回收站中看到备份.

**Alternative**: 移动到临时目录. 但需要自行管理清理, 且跨设备时可能失败.

### Decision 4: UndoManager 增加 push_copy_pair 和 push_move_pair 方法

类似 `push_trash_pair`, 追加 pair 到最新的 Copy/Move entry. 同时增加 `push_copy_overwritten` 和 `push_move_overwritten` 方法追加 overwritten 对.

**Why**: 保持与 trash 一致的增量式 pair 收集模式.

## Risks / Trade-offs

- **竞态**: 快速连续粘贴可能导致 hook 回报到错误的 undo entry → 与 trash 存在相同限制, 实际使用中极少发生. 缓解: 文档记录此限制.
- **磁盘占用**: 强制覆盖会在回收站保留原文件 → 用户可清理回收站. 这与删除文件到回收站的行为一致.
- **跨文件系统**: `provider::trash()` 在跨文件系统时可能失败 → 静默跳过 trash, 降级为当前行为(覆盖不可恢复).
