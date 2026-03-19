## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: Tab action disabled in dual-pane mode
**This requirement is removed.**

## REMOVED Requirements

### Requirement: Tab action disabled in dual-pane mode
**Reason**: Tab operations are now fully supported, scoped to the active pane.  Each pane maintains its own independent tab list.
**Migration**: All tab operations (create, close, switch, swap, rename) now work as expected within the active pane.  No migration needed.
