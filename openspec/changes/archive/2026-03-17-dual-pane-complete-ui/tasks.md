## 1. Modify DualPane to include per-pane Header and Status

- [x] 1.1 Update `dual_pane.lua` layout to create vertical sub-layouts per pane: each pane column splits into `[Length(1), Fill(1), Length(1)]` for header, tab content, and status
- [x] 1.2 Update `dual_pane.lua` build to instantiate `Header:new()` and `Status:new()` for each pane, passing the respective tab (`cx.tabs[1]` and `cx.tabs[2]`)
- [x] 1.3 Update `dual_pane.lua` reflow and redraw to include the per-pane Header and Status components

## 2. Modify Root to conditionally skip header/status in dual-pane mode

- [x] 2.1 Update `root.lua` layout to use a single `Fill(1)` constraint when `cx.tabs.single_pane` is false, instead of `[Length(1), Fill(1), Length(1)]`
- [x] 2.2 Update `root.lua` build to omit root-level Header and Status children when in dual-pane mode, passing the full area to DualPane

## 3. Verify and test

- [x] 3.1 Verify dual-pane mode shows independent header and status for each pane
- [x] 3.2 Verify single-pane mode still renders correctly with root-level header and status
- [x] 3.3 Verify pane switching updates active indicator without losing per-pane UI elements
