## 1. Core Data Layer (yazi-core)

- [x] 1.1 Modify `Mgr::make()` in `yazi-core/src/mgr/mgr.rs` to initialize `Tabs` with exactly 2 `Tab` items instead of 1
- [x] 1.2 Add `other()` and `other_mut()` methods to `Tabs` in `yazi-core/src/mgr/tabs.rs` that return the non-active tab

## 2. Action Parsers (yazi-parser)

- [x] 2.1 Create `yazi-parser/src/mgr/pane_switch.rs` with `PaneSwitchOpt` (no options needed)
- [x] 2.2 Create `yazi-parser/src/mgr/pane_focus.rs` with `PaneFocusOpt { left: bool }` for directional focus
- [x] 2.3 Create `yazi-parser/src/mgr/copy_to.rs` with `CopyToOpt { force: bool }` for F5
- [x] 2.4 Create `yazi-parser/src/mgr/move_to.rs` with `MoveToOpt { force: bool }` for F6
- [x] 2.5 Register all new parsers in `yazi-parser/src/mgr/mod.rs`

## 3. Actions (yazi-actor)

- [x] 3.1 Create `yazi-actor/src/mgr/pane_switch.rs` implementing `pane_switch` action: toggle `tabs.cursor` between 0 and 1
- [x] 3.2 Create `yazi-actor/src/mgr/pane_focus.rs` implementing `pane_focus` action: set `tabs.cursor` to 0 (left) or 1 (right)
- [x] 3.3 Create `yazi-actor/src/mgr/copy_to.rs` implementing `copy_to` action: get selected files from active tab, get other tab's cwd as dest, call `tasks.file_copy`, clear selection
- [x] 3.4 Create `yazi-actor/src/mgr/move_to.rs` implementing `move_to` action: same as copy_to but call `tasks.file_cut` and unyank
- [x] 3.5 Register all new actors in `yazi-actor/src/mgr/mod.rs`
- [x] 3.6 Add guard to `tab_create`, `tab_close`, `tab_switch`, `tab_swap`, `tab_rename` actors to make them no-ops when tabs.len() == 2 (dual-pane mode)

## 4. Action Routing (yazi-fm)

- [x] 4.1 Add `pane_switch`, `pane_focus`, `copy_to`, `move_to` to the `mgr` match block in `yazi-fm/src/executor.rs`

## 5. Lua UI Components (yazi-plugin)

- [x] 5.1 Create `yazi-plugin/preset/components/dual_pane.lua` with DualPane component: horizontal split into left (Ratio 1/2) + separator (Length 1) + right (Ratio 1/2), each rendering a `Tab` instance
- [x] 5.2 Create separator rendering in `dual_pane.lua` (vertical line character `│`)
- [x] 5.3 Modify `yazi-plugin/preset/components/root.lua`: replace `Tabs:new()` + `Tab:new()` with `DualPane:new()`, remove tab bar height from layout constraints
- [x] 5.4 Add active/inactive pane visual distinction: inactive pane renders with dimmed style or border indicator

## 6. Configuration and Keybindings

- [x] 6.1 Update `yazi-config/preset/keymap-default.toml` to add `Tab` → `pane_switch`, `Ctrl-w w` → `pane_switch`, `Ctrl-w Ctrl-w` → `pane_switch`, `Ctrl-w h` → `pane_focus --left`, `Ctrl-w l` → `pane_focus --right`
- [x] 6.2 Update `yazi-config/preset/keymap-default.toml` to add `F5` → `copy_to`, `F6` → `move_to`, `F7` → `create`, `F8` → `remove`
- [x] 6.3 Remove or comment out existing `tab_create`, `tab_close`, `tab_switch`, `tab_swap` keybindings from keymap-default.toml
- [x] 6.4 Set default ratio to `[1, 2, 0]` in `yazi-config/preset/yazi-default.toml` for dual-pane mode

## 7. Build and Verification

- [x] 7.1 Ensure project compiles with `cargo build`
- [x] 7.2 Verify dual-pane layout renders correctly on startup
- [x] 7.3 Verify Tab key switches between panes
- [x] 7.4 Verify F5 copies selected files to the other pane's directory
- [x] 7.5 Verify F6 moves selected files to the other pane's directory
- [x] 7.6 Verify existing yank/paste workflow works across panes
