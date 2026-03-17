## Why

dual-pane-mode 变更将 dual-yazi 固定为双栏模式并隐藏了 preview 列.  但有些场景下用户需要回到单栏全宽视图来浏览文件预览 (如查看代码, 图片等), 然后再切回双栏继续跨栏操作.  需要一个自然的模式切换机制, 类似 vifm 的 `:only` / `Ctrl-w o` 但支持双向切换.

## What Changes

- 新增 `pane_only` action, 将布局从双栏切换为单栏 (只显示活跃 pane, 恢复 preview)
- 单栏模式下 ratio 恢复为带 preview 的配置 (如 `[1, 4, 3]`), 非活跃 pane 的状态保留但不渲染
- 再次触发 `pane_only` 时切回双栏模式 (toggle 行为)
- 绑定 `Ctrl-w o` 为切换快捷键, 与 vim/vifm 的 `:only` 语义一致
- Lua UI 层的 `root.lua` 根据当前模式选择渲染 DualPane 组件或单个 Tab 组件

## Capabilities

### New Capabilities
- `pane-mode-toggle`: 双栏/单栏模式切换, 包括 pane_only action, 布局切换逻辑, ratio 动态调整, 快捷键绑定

### Modified Capabilities
- `dual-pane-layout`: root.lua 需要根据模式状态选择 DualPane 或单 Tab 渲染

## Impact

- `yazi-core/src/mgr/`: Mgr 或 Tabs 新增 `single_pane: bool` 状态字段
- `yazi-actor/src/mgr/`: 新增 pane_only action
- `yazi-parser/src/mgr/`: 新增 PaneOnlyOpt 解析器
- `yazi-fm/src/executor.rs`: 路由 pane_only action
- `yazi-actor/src/lives/core.rs`: 暴露 single_pane 状态到 Lua
- `yazi-plugin/preset/components/root.lua`: 根据模式选择布局
- `yazi-config/preset/keymap-default.toml`: 绑定 Ctrl-w o
