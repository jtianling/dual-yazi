## Why

当前 `t` 键被用作组合键前缀(`tt` 创建 tab, `tr` 重命名 tab), 与原版 yazi 的行为不一致.  原版 yazi 中 `t` 单键直接创建新 tab, `r` 单键重命名文件.  当前配置导致按 `t` 后需要等待第二个键, 影响使用体验.

## What Changes

- 将 `tab_create` 的键绑定从 `["t", "t"]` 改为单键 `"t"`, 与原版 yazi 一致
- 移除 `["t", "r"]` 的 `tab_rename` 键绑定(原版 yazi 默认不绑定 tab_rename)
- `r` 键的 `rename` 绑定保持不变(已与原版一致)

## Capabilities

### New Capabilities

(无)

### Modified Capabilities

- `pane-keybindings`: tab 相关键绑定需要更新, 恢复 `t` 单键创建 tab 的行为

## Impact

- `yazi-config/preset/keymap-default.toml` 中两行键绑定变更
- 不涉及 Rust 代码改动, 仅配置文件变更
