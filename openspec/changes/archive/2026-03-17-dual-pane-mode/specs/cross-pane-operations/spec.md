## ADDED Requirements

### Requirement: Copy to other pane (F5)
The system SHALL provide a `copy_to` action that copies the selected files (or hovered file if none selected) from the active pane directly to the other pane's current working directory.  This operation SHALL NOT use the yank register.  After the operation, the selection state SHALL be cleared.

#### Scenario: Copy single file to other pane
- **WHEN** the user has no files selected and the cursor is on "file.txt" in the left pane, and the right pane is in /tmp
- **THEN** pressing F5 copies file.txt to /tmp/file.txt

#### Scenario: Copy multiple selected files to other pane
- **WHEN** the user has 3 files selected in the left pane, and the right pane is in /home/user/backup
- **THEN** pressing F5 copies all 3 files to /home/user/backup/

#### Scenario: Copy clears selection
- **WHEN** the user has files selected and presses F5
- **THEN** the files are copied and the selection state is cleared in the source pane

### Requirement: Move to other pane (F6)
The system SHALL provide a `move_to` action that moves the selected files (or hovered file if none selected) from the active pane directly to the other pane's current working directory.  This operation SHALL NOT use the yank register.  After the operation, the selection state SHALL be cleared.

#### Scenario: Move single file to other pane
- **WHEN** the user has no files selected and the cursor is on "file.txt" in the left pane, and the right pane is in /tmp
- **THEN** pressing F6 moves file.txt to /tmp/file.txt

#### Scenario: Move multiple selected files to other pane
- **WHEN** the user has 3 files selected in the left pane, and the right pane is in /home/user/archive
- **THEN** pressing F6 moves all 3 files to /home/user/archive/

### Requirement: Create directory (F7)
The system SHALL bind F7 to the existing `create` action, providing MC-compatible directory creation.

#### Scenario: Create directory with F7
- **WHEN** the user presses F7
- **THEN** the create dialog appears, allowing the user to create a new file or directory

### Requirement: Delete files (F8)
The system SHALL bind F8 to the existing `remove` action, providing MC-compatible file deletion.

#### Scenario: Delete with F8
- **WHEN** the user presses F8
- **THEN** the remove confirmation appears for the selected/hovered files

### Requirement: Existing yank/paste cross-pane compatibility
The existing yank (`y`) and paste (`p`) workflow SHALL continue to work across panes.  When the user yanks in one pane, switches to the other pane, and pastes, the paste destination SHALL be the active pane's current working directory.

#### Scenario: Yank in left, paste in right
- **WHEN** the user yanks files in the left pane, switches to the right pane, and pastes
- **THEN** files are pasted to the right pane's current directory
