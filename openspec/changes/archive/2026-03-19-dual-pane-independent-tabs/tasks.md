## 1. Core Data Structure Refactor

- [x] 1.1 Create `Pane` struct in `yazi-core/src/mgr/` containing `tabs: Vec<Tab>`, `cursor: usize`, `single_pane: bool`, `preview_pane: bool`, and provide `active()`, `active_mut()`, `len()`, `set_idx()` methods
- [x] 1.2 Refactor `Mgr` to hold `panes: [Pane; 2]` and `active_pane: usize` instead of `tabs: Tabs`; add `active_pane()`, `other_pane()`, `active_pane_mut()`, `other_pane_mut()` convenience methods
- [x] 1.3 Update `Ctx` compatibility layer: ensure `cx.tab()`, `cx.tabs()`, `cx.tabs_mut()`, `cx.cwd()`, `cx.hovered()` etc. proxy to active pane's tabs
- [x] 1.4 Remove old `Tabs` struct or repurpose it; clean up `Deref`/`DerefMut` impls

## 2. Tab Actor Adaptation

- [x] 2.1 Update `tab_create.rs`: remove the `len() == 2` early return guard, operate on active pane's tab list
- [x] 2.2 Update `tab_close.rs`: remove the `len() == 2` early return guard, prevent close only when pane has 1 tab
- [x] 2.3 Update `tab_switch.rs`: remove the `len() == 2` early return guard, switch within active pane's tab list
- [x] 2.4 Update `tab_swap.rs`: remove guard, swap within active pane's tab list
- [x] 2.5 Update `tab_rename.rs`: remove guard, rename within active pane's tab list

## 3. Pane Actor Adaptation

- [x] 3.1 Update `pane_switch.rs`: switch active pane index (0 <-> 1) using new `Mgr.active_pane` field
- [x] 3.2 Update `pane_focus.rs`: set active pane by direction using new field
- [x] 3.3 Update `pane_only.rs` and `pane_preview.rs`: toggle mode flags on the active pane (or global, per design)
- [x] 3.4 Update `copy_to.rs` and `move_to.rs`: target `cx.other_pane().active().cwd()` instead of `cx.tabs().other().cwd()`

## 4. System-wide Actor Updates

- [x] 4.1 Update `bootstrap.rs`: initialize 2 panes each with 1 tab instead of 2 flat tabs
- [x] 4.2 Update `update_files.rs`: iterate all panes and all tabs within each pane
- [x] 4.3 Update `watch.rs`: collect watched paths from all panes and all tabs
- [x] 4.4 Update `yazi-fm/src/mgr/preview.rs`: adapt to new pane structure for other pane's preview

## 5. Executor and Spark Registration

- [x] 5.1 Verify `yazi-fm/src/executor.rs` dispatches tab actions correctly with new structure (likely no changes needed if Ctx compatibility layer works)
- [x] 5.2 Verify `yazi-dds/src/spark/spark.rs` enum and `try_from_spark!` entries are correct (likely no changes needed)

## 6. Lua Bindings and UI

- [x] 6.1 Update `yazi-actor/src/lives/tabs.rs`: expose per-pane tab data to Lua, including pane-level `idx`, `len`, tab list
- [x] 6.2 Update `dual_pane.lua`: pass pane's tab list to child components; conditionally render tab bar per pane
- [x] 6.3 Update `root.lua`: in single-pane mode pass active pane's tab data
- [x] 6.4 Add tab bar rendering within each pane in `dual_pane.lua` (reuse existing tab bar component or create minimal one)

## 7. Keymap Restoration

- [x] 7.1 Restore tab management keybindings in `keymap-default.toml`: `t` for tab_create, `1-9` for tab_switch, `[`/`]` for tab_switch relative, `{`/`}` for tab_swap

## 8. Verification

- [x] 8.1 `cargo check` passes with no errors
- [x] 8.2 Manual test: create/close/switch tabs independently in each pane
- [ ] 8.3 Manual test: copy_to/move_to targets the other pane's active tab
- [x] 8.4 Manual test: single-pane mode tab operations work as original yazi
