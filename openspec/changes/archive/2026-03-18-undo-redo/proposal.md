## Why

File managers should support undo/redo for file operations, matching vim's `u` / `<C-r>` paradigm. Currently, keybindings for `undo` and `redo` exist in the default keymap but silently no-op because no Actor implementation exists. Destructive file operations (rename, move, delete) are irreversible, which is a poor UX for a vim-oriented tool.

## What Changes

- New `UndoManager` data structure in `Mgr` with undo/redo stacks (max 20 entries)
- New `Undo` and `Redo` Actors in the Mgr layer that execute reverse/forward operations
- Existing file operation Actors (`rename`, `create`, `paste`, `copy_to`, `move_to`) modified to record undo entries optimistically at action time
- New parser options (`UndoOpt`, `RedoOpt`) and executor registration
- First version covers: rename, create, copy (paste), move (cut+paste, copy_to, move_to)
- Trash restore and permanent delete are explicitly out of scope for this change

## Capabilities

### New Capabilities
- `undo-redo`: Core undo/redo stack management and execution of reverse file operations via `u` and `<C-r>` keybindings

### Modified Capabilities

(none — no existing spec-level behavior changes)

## Impact

- **Code**: yazi-core (UndoManager in Mgr), yazi-actor (new undo/redo actors + modify 5 existing actors), yazi-parser (new opt types), yazi-fm (executor registration), yazi-proxy (new proxy methods)
- **Dependencies**: No new crate dependencies
- **Keybindings**: Already configured in keymap-default.toml, no changes needed
