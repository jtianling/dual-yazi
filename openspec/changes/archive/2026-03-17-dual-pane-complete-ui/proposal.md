## Why

In dual-pane mode, the Header (directory path), Status bar (mode + filename), and file attributes (permissions + position) are rendered only once for the active pane. This creates an asymmetric layout where one pane lacks essential context — the user cannot see the directory path, mode indicator, or file permissions for the inactive pane without switching to it. Each pane should be a self-contained unit with its own header and status bar.

## What Changes

- Move Header and Status rendering from the root level into each pane in dual-pane mode
- In dual-pane mode, each pane will have its own:
  - Header row showing that pane's current directory path and file counts
  - Status bar row showing that pane's mode, file name, permissions, and position
- In single-pane mode, behavior remains unchanged (single header and status at root level)
- The overall root layout in dual-pane mode changes from `[header | dual-pane | status]` to `[dual-pane]` where each pane internally contains `[header | tab-content | status]`

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `dual-pane-layout`: The "Header and status bar adaptation" requirement changes from showing only active pane info to each pane having its own independent header and status bar

## Impact

- `yazi-plugin/preset/components/root.lua`: Conditional layout — skip root-level header/status in dual-pane mode
- `yazi-plugin/preset/components/dual_pane.lua`: Each pane sub-layout becomes vertical `[header | tab | status]` instead of just `[tab]`
- `yazi-plugin/preset/components/header.lua`: No structural changes needed, already accepts a tab parameter
- `yazi-plugin/preset/components/status.lua`: No structural changes needed, already accepts a tab parameter
