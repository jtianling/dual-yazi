## ADDED Requirements

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

### Requirement: Tab action disabled in dual-pane mode
The system SHALL disable `tab_create`, `tab_close`, `tab_switch`, `tab_swap`, and `tab_rename` actions.  These actions SHALL be no-ops when invoked.

#### Scenario: Tab create is no-op
- **WHEN** the user or a plugin invokes tab_create
- **THEN** no new tab is created and the system continues normally

#### Scenario: Tab close is no-op
- **WHEN** the user or a plugin invokes tab_close
- **THEN** no pane is closed and the system continues normally
