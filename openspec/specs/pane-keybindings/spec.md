# Capability: Pane Keybindings

## Purpose

Defines keyboard shortcuts for pane navigation and cross-pane operations in dual-pane mode.

## Requirements

### Requirement: Tab key switches active pane
The system SHALL bind the `Tab` key to `pane_switch` action, which toggles the active pane between left and right.

#### Scenario: Switch from left to right
- **WHEN** the left pane is active and the user presses Tab
- **THEN** the right pane becomes active

#### Scenario: Switch from right to left
- **WHEN** the right pane is active and the user presses Tab
- **THEN** the left pane becomes active

### Requirement: Ctrl-w w switches active pane
The system SHALL bind `Ctrl-w w` and `Ctrl-w Ctrl-w` to `pane_switch` action.

#### Scenario: Ctrl-w w toggle
- **WHEN** the user presses Ctrl-w followed by w
- **THEN** the active pane switches to the other pane

#### Scenario: Ctrl-w Ctrl-w toggle
- **WHEN** the user presses Ctrl-w followed by Ctrl-w
- **THEN** the active pane switches to the other pane

### Requirement: Ctrl-w h/l directional pane focus
The system SHALL bind `Ctrl-w h` to focus the left pane and `Ctrl-w l` to focus the right pane, regardless of the currently active pane.

#### Scenario: Focus left pane
- **WHEN** the right pane is active and the user presses Ctrl-w h
- **THEN** the left pane becomes active

#### Scenario: Focus right pane when already on right
- **WHEN** the right pane is active and the user presses Ctrl-w l
- **THEN** the right pane remains active (no-op)

#### Scenario: Focus right pane from left
- **WHEN** the left pane is active and the user presses Ctrl-w l
- **THEN** the right pane becomes active

### Requirement: MC-style function keys
The system SHALL bind F5, F6, F7, F8 to cross-pane operations as defined in the cross-pane-operations capability.

#### Scenario: F5 copies to other pane
- **WHEN** the user presses F5
- **THEN** the `copy_to` action is executed

#### Scenario: F6 moves to other pane
- **WHEN** the user presses F6
- **THEN** the `move_to` action is executed

#### Scenario: F7 creates directory
- **WHEN** the user presses F7
- **THEN** the `create` action is executed

#### Scenario: F8 deletes files
- **WHEN** the user presses F8
- **THEN** the `remove` action is executed

### Requirement: Space key preserves toggle behavior
The system SHALL NOT rebind the Space key.  Space SHALL continue to toggle selection state and move the cursor down, as in the original yazi.

#### Scenario: Space toggles selection
- **WHEN** the user presses Space on a file
- **THEN** the file's selection state is toggled and the cursor moves down one item

### Requirement: Ctrl-w p toggles preview mode
The system SHALL bind `Ctrl-w p` to the `pane_preview` action, which toggles the dual-pane layout between directory mode and preview mode.

#### Scenario: Ctrl-w p toggles preview
- **WHEN** the user presses Ctrl-w followed by p
- **THEN** the dual-pane layout toggles between directory mode (parent + current) and preview mode (current + preview)

### Requirement: Tab create keybinding
The system SHALL bind `t` to `tab_create --current`, which creates a new tab in the active pane at the current directory.

#### Scenario: Create tab with t key
- **WHEN** the user presses t
- **THEN** a new tab is created in the active pane at the current directory

### Requirement: Tab switch by number
The system SHALL bind keys `1` through `9` to `tab_switch <n>` (0-indexed), which switches to the nth tab within the active pane.

#### Scenario: Switch to tab 1
- **WHEN** the user presses 1
- **THEN** the first tab (index 0) in the active pane becomes the active tab

#### Scenario: Switch to tab 3
- **WHEN** the active pane has 3 or more tabs and the user presses 3
- **THEN** the third tab (index 2) in the active pane becomes the active tab

### Requirement: Tab switch relative keybinding
The system SHALL bind `[` to `tab_switch -1 --relative` and `]` to `tab_switch 1 --relative`, which switch to the previous/next tab within the active pane.

#### Scenario: Switch to previous tab
- **WHEN** the user presses [
- **THEN** the previous tab in the active pane becomes the active tab

#### Scenario: Switch to next tab
- **WHEN** the user presses ]
- **THEN** the next tab in the active pane becomes the active tab

### Requirement: Tab swap keybinding
The system SHALL bind `{` to `tab_swap -1` and `}` to `tab_swap 1`, which swap the active tab position within the active pane.

#### Scenario: Swap tab left
- **WHEN** the user presses {
- **THEN** the active tab swaps position with the previous tab within the pane

#### Scenario: Swap tab right
- **WHEN** the user presses }
- **THEN** the active tab swaps position with the next tab within the pane
