## Why

Yazi 是一个优秀的终端文件管理器, 但只支持单栏 miller columns 布局.  对于习惯 vifm/Midnight Commander 双栏操作的用户, 缺少左右两栏并排浏览和跨栏文件操作的能力, 使得批量拷贝/移动文件需要频繁切换目录.  本变更将 yazi 改造为 dual-yazi, 提供双栏文件管理体验.

## What Changes

- 启动时初始化两个并排的 pane, 替代原有的 tab bar 布局
- **BREAKING**: 移除多 tab 支持, 固定为左右两个 pane
- 新增 `pane_switch` action, 通过 `Tab` / `Ctrl-w` 系列快捷键切换活跃栏
- 新增 `copy_to` / `move_to` action, 直接将选中文件复制/移动到另一栏目录 (MC 风格, 不经过 yank 寄存器)
- 双栏模式下默认使用 `ratio = [1, 2, 0]` 隐藏 preview 列, 每栏显示 parent + current
- 绑定 F5 (copy to other), F6 (move to other), F7 (mkdir), F8 (delete) 等 MC 风格快捷键
- Lua UI 层新增 DualPane 组件, 负责左右栏布局和分隔线渲染
- 非活跃栏视觉降低亮度或调整样式以区分焦点

## Capabilities

### New Capabilities
- `dual-pane-layout`: 双栏布局系统, 包括 pane 初始化, 分隔线, 活跃栏切换, 以及 Lua DualPane 组件
- `cross-pane-operations`: 跨栏文件操作, 包括 MC 风格的 copy_to/move_to (F5/F6) 以及 F7/F8 快捷键
- `pane-keybindings`: 双栏模式下的快捷键绑定, 包括 Tab/Ctrl-w 系列切换和 MC 风格功能键

### Modified Capabilities

## Impact

- `yazi-core/src/mgr/`: Mgr 和 Tabs 结构需要适配固定双 pane 模式
- `yazi-actor/src/mgr/`: 新增 pane_switch, copy_to, move_to action; 禁用 tab_create/tab_close/tab_switch/tab_swap/tab_rename
- `yazi-actor/src/lives/`: Lua bridge 需要暴露双栏状态 (cx.panes 或复用 cx.tabs)
- `yazi-parser/src/mgr/`: 新增对应 action 的 option 解析器
- `yazi-fm/src/executor.rs`: 路由新 action
- `yazi-fm/src/router.rs`: 可能需要适配 pane 切换的按键路由
- `yazi-plugin/preset/components/`: root.lua, tab.lua 重构; 新增 dual_pane.lua
- `yazi-config/preset/keymap-default.toml`: 新增快捷键绑定
- `yazi-config/src/mgr/ratio.rs`: 双栏模式下默认 ratio 调整
