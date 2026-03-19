## 1. Keymap Configuration

- [x] 1.1 将 `keymap-default.toml` 中 `tab_create` 的绑定从 `[ "t", "t" ]` 改为 `"t"`
- [x] 1.2 移除 `keymap-default.toml` 中 `[ "t", "r" ]` 的 `tab_rename --interactive` 绑定

## 2. Verification

- [x] 2.1 `cargo check` 编译通过
- [x] 2.2 确认 `r` 键仍绑定为 `rename --cursor=before_ext`(无变化)
