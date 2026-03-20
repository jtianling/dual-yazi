## 1. Shift+F5 / Shift+F6 快捷键绑定

- [x] 1.1 在 `yazi-config/preset/keymap-default.toml` 中添加 `<S-F5>` → `copy_to --force` 和 `<S-F6>` → `move_to --force` 键绑定

## 2. pane_sync_dir action 实现

- [x] 2.1 在 `yazi-parser/src/mgr/` 中创建 `pane_sync_dir.rs`, 定义 `PaneSyncDirOpt` (无参数)
- [x] 2.2 在 `yazi-parser/src/mgr/mod.rs` 中注册 `PaneSyncDirOpt`
- [x] 2.3 在 `yazi-actor/src/mgr/` 中创建 `pane_sync_dir.rs`, 实现 `PaneSyncDir` actor (切换栏 → cd → 切回)
- [x] 2.4 在 `yazi-actor/src/mgr/mod.rs` 中注册 `PaneSyncDir`
- [x] 2.5 在 `yazi-fm/src/executor.rs` 中注册 `pane_sync_dir` action

## 3. = 快捷键绑定

- [x] 3.1 在 `yazi-config/preset/keymap-default.toml` 中添加 `=` → `pane_sync_dir` 键绑定
