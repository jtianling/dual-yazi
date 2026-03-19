## Why

当前双面板模式硬编码为恰好 2 个 tab (左右各一个), 并禁用了所有 tab 管理操作 (create/close/switch/swap/rename).  这丧失了原版 yazi 的核心功能.  用户需要在双面板的同时, 在每个面板内独立管理多个 tab, 就像两个独立的 yazi 实例.

## What Changes

- **BREAKING**: `Tabs` 数据结构从扁平 `Vec<Tab>` 改为双面板各自独立的 tab 列表, 每个面板有自己的 cursor
- 恢复 `tab_create`, `tab_close`, `tab_switch`, `tab_swap`, `tab_rename` 操作, 作用于当前活跃面板的 tab 列表
- 恢复 keymap-default.toml 中原有的 tab 管理键位 (`t` 创建, `1-9` 切换, `[`/`]` swap 等)
- 更新 Lua UI 层, 在每个面板内显示多 tab 时的 tab bar
- `pane_switch` / `pane_focus` 切换面板时, 操作的是对面面板的活跃 tab
- `copy_to` / `move_to` 等跨面板操作的目标为对面面板的活跃 tab 的 cwd

## Capabilities

### New Capabilities
- `per-pane-tabs`: 每个面板独立的 tab 管理, 包括数据结构、tab CRUD 操作在面板作用域内的行为

### Modified Capabilities
- `dual-pane-layout`: 布局需要在每个面板内支持多 tab 时的 tab bar 显示
- `pane-keybindings`: 恢复 tab 管理键位, 移除 "tab 操作被禁用" 的约束
- `cross-pane-operations`: 跨面板操作的目标改为对面面板的活跃 tab 的 cwd

## Impact

- `yazi-core/src/mgr/tabs.rs` — 核心数据结构重构
- `yazi-actor/src/mgr/tab_*.rs` — 所有 tab actor 需要改为在面板作用域内操作
- `yazi-actor/src/mgr/copy_to.rs`, `move_to.rs` — 目标 cwd 取法变更
- `yazi-actor/src/app/bootstrap.rs` — 初始化逻辑适配新结构
- `yazi-actor/src/mgr/update_files.rs`, `watch.rs` — 遍历所有 tab 的逻辑适配
- `yazi-plugin/preset/components/dual_pane.lua`, `root.lua` — UI 渲染适配 tab bar
- `yazi-config/preset/keymap-default.toml` — 恢复 tab 管理键位
- `yazi-dds/src/spark/spark.rs`, `yazi-fm/src/executor.rs` — 如有新增 action 需注册
- `yazi-actor/src/lives/tabs.rs` — Lua 绑定适配新结构
