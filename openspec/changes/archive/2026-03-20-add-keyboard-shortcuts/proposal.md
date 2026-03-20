## Why

dual-yazi 的 F5 (copy_to) 和 F6 (move_to) 默认使用非覆盖模式(遇到同名文件会重命名为 `_1` 后缀).  用户需要一种快捷方式直接执行覆盖性复制/移动, 而不需要额外确认.  同时, 双栏文件管理器中经常需要让两侧显示同一目录以便比较或操作, 目前缺少这样的快捷键.

## What Changes

- 新增 `Shift+F5` 快捷键, 执行覆盖性 copy_to (等同于 `copy_to --force`)
- 新增 `Shift+F6` 快捷键, 执行覆盖性 move_to (等同于 `move_to --force`)
- 新增 `=` 快捷键, 让对面栏导航到当前栏的目录
- 新增 `pane_sync_dir` action, 实现跨栏目录同步

## Capabilities

### New Capabilities
- `pane-sync-dir`: 新 action, 让对面栏切换到当前栏的工作目录

### Modified Capabilities
- `cross-pane-operations`: 增加 Shift+F5 (force copy) 和 Shift+F6 (force move) 快捷键
- `pane-keybindings`: 增加 `=` 键绑定到 pane_sync_dir action

## Impact

- `yazi-config/preset/keymap-default.toml`: 新增三个键绑定
- `yazi-actor/src/mgr/`: 新增 pane_sync_dir actor
- `yazi-parser/src/mgr/`: 新增 pane_sync_dir parser
- `yazi-fm/src/executor.rs`: 注册新 action
- `yazi-proxy/src/mgr.rs`: 可能需要新增 proxy 方法
