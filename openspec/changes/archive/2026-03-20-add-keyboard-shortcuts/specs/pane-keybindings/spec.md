## ADDED Requirements

### Requirement: Shift+F5 force copies to other pane
The system SHALL bind `Shift+F5` to `copy_to --force`, which copies selected files to the other pane with overwrite enabled.

#### Scenario: Shift+F5 executes force copy
- **WHEN** the user presses Shift+F5
- **THEN** the `copy_to --force` action is executed

### Requirement: Shift+F6 force moves to other pane
The system SHALL bind `Shift+F6` to `move_to --force`, which moves selected files to the other pane with overwrite enabled.

#### Scenario: Shift+F6 executes force move
- **WHEN** the user presses Shift+F6
- **THEN** the `move_to --force` action is executed

### Requirement: Equals key syncs directory to other pane
The system SHALL bind `=` to `pane_sync_dir`, which navigates the other pane to the current pane's directory.

#### Scenario: Equals key syncs directory
- **WHEN** the user presses `=`
- **THEN** the `pane_sync_dir` action is executed, and the other pane navigates to the current pane's directory
