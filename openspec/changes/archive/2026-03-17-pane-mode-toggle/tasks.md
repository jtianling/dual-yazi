## 1. Core Data Layer (yazi-core)

- [x] 1.1 Add `pub single_pane: bool` field to `Tabs` struct in `yazi-core/src/mgr/tabs.rs`, default `false`

## 2. Action Parser (yazi-parser)

- [x] 2.1 Create `yazi-parser/src/mgr/pane_only.rs` with `PaneOnlyOpt` (no options needed, toggle behavior)
- [x] 2.2 Register `pane_only` parser in `yazi-parser/src/mgr/mod.rs`

## 3. Action (yazi-actor)

- [x] 3.1 Create `yazi-actor/src/mgr/pane_only.rs` implementing `pane_only` action: toggle `tabs.single_pane` between true and false, trigger render
- [x] 3.2 Register `pane_only` actor in `yazi-actor/src/mgr/mod.rs`

## 4. Action Routing (yazi-fm)

- [x] 4.1 Add `pane_only` to the `mgr` match block in `yazi-fm/src/executor.rs`

## 5. Lua Bridge (yazi-actor/lives)

- [x] 5.1 Expose `single_pane` field on the Tabs userdata in `yazi-actor/src/lives/tabs.rs` so Lua can access `cx.tabs.single_pane`

## 6. Lua UI Components (yazi-plugin)

- [x] 6.1 Modify `yazi-plugin/preset/components/root.lua`: check `cx.tabs.single_pane` to decide between rendering `DualPane:new()` or single `Tab:new()` with full-width area
- [x] 6.2 In single-pane mode, use `rt.mgr.ratio` (user-configured ratio with preview) instead of DualPane's hardcoded `[1, 2, 0]`

## 7. Keybindings

- [x] 7.1 Add `Ctrl-w o` → `pane_only` binding in `yazi-config/preset/keymap-default.toml`

## 8. Build and Verification

- [x] 8.1 Ensure project compiles with `cargo build`
- [x] 8.2 Verify Ctrl-w o toggles from dual-pane to single-pane (active pane full width with preview)
- [x] 8.3 Verify Ctrl-w o toggles back from single-pane to dual-pane
- [x] 8.4 Verify Tab key still switches active pane in single-pane mode (view changes to other pane's directory)
- [x] 8.5 Verify F5/F6 still work in single-pane mode (copy/move to hidden pane's directory)
- [x] 8.6 Verify inactive pane state preserved across mode toggles
