## Context

dual-yazi 已有 F5 (`copy_to`) 和 F6 (`move_to`) 绑定, 底层操作已支持 `--force` 参数用于覆盖同名文件.  目前缺少覆盖模式的快捷键绑定和跨栏目录同步功能.

## Goals / Non-Goals

**Goals:**
- 为 `copy_to --force` 和 `move_to --force` 添加 Shift+F5/F6 快捷键
- 实现 `pane_sync_dir` action, 让对面栏导航到当前栏的目录
- 为 `pane_sync_dir` 添加 `=` 快捷键

**Non-Goals:**
- 不修改 copy_to/move_to 的底层逻辑, 仅利用已有的 force 参数
- 不做双向同步(不改变当前活动栏)

## Decisions

### 1. Shift+F5/F6 复用已有的 force 参数

copy_to 和 move_to 的 actor 已经接受 `force` 选项, 只需在 keymap 中添加 `copy_to --force` 和 `move_to --force` 的快捷键绑定即可.  无需修改任何 actor/parser 代码.

**替代方案**: 创建独立的 `force_copy_to` / `force_move_to` action → 过度复杂, 不必要.

### 2. pane_sync_dir 实现策略: 切换-cd-切回

实现步骤:
1. 记录当前栏的 cwd
2. 切换到对面栏 (通过 `pane_switch`)
3. 在对面栏执行 `cd` 到保存的目录
4. 切回原来的栏 (通过 `pane_switch`)

这样复用了已有的 `cd` 和 `pane_switch` 逻辑, 确保所有副作用(Pubsub 通知, 刷新, 渲染等)正确触发.

**替代方案**: 直接操作对面栏的 tab 数据结构 → 需要处理 history, parent, 事件发布等所有 cd 副作用, 容易遗漏.

### 3. Actor 模式遵循现有 pane_* 模式

pane_sync_dir 遵循与 pane_switch 相同的模式:
- `yazi-parser/src/mgr/pane_sync_dir.rs`: 无参数的 Options 结构体
- `yazi-actor/src/mgr/pane_sync_dir.rs`: Actor 实现
- 在 mod.rs 和 executor.rs 中注册

## Risks / Trade-offs

- **切换闪烁**: 切换-cd-切回可能导致短暂的 UI 闪烁 → 由于切换和 cd 都在同一帧内完成(同步操作), 不会造成可见的闪烁.
- **当前目录相同时**: 如果两栏已在相同目录, cd 会提前返回 (已有 `if opt.target == *tab.cwd()` 检查), 为安全的 no-op.
