# Capability: Cross Pane Operations

## Purpose

Defines file operations that work across panes, including direct copy/move to the other pane and MC-compatible function key bindings.

## Requirements

### Requirement: Copy to other pane (F5)
The system SHALL provide a `copy_to` action that copies the selected files (or hovered file if none selected) from the active pane's active tab to the other pane's active tab's current working directory.  This operation SHALL NOT use the yank register.  After the operation, the selection state SHALL be cleared.

#### Scenario: Copy single file to other pane
- **WHEN** the user has no files selected and the cursor is on "file.txt" in the left pane, and the right pane is in /tmp
- **THEN** pressing F5 copies file.txt to /tmp/file.txt

#### Scenario: Copy multiple selected files to other pane
- **WHEN** the user has 3 files selected in the left pane, and the right pane is in /home/user/backup
- **THEN** pressing F5 copies all 3 files to /home/user/backup/

#### Scenario: Copy clears selection
- **WHEN** the user has files selected and presses F5
- **THEN** the files are copied and the selection state is cleared in the source pane

#### Scenario: Copy files to other pane's active tab
- **WHEN** the user selects files in the left pane's active tab and presses F5
- **THEN** the files are copied to the right pane's active tab's current working directory

#### Scenario: Copy when other pane has multiple tabs
- **WHEN** the right pane has 3 tabs with tab 2 active, and the user copies from the left pane
- **THEN** the files are copied to the right pane's tab 2's current working directory (not tab 1 or tab 3)

### Requirement: Move to other pane (F6)
The system SHALL provide a `move_to` action that moves the selected files (or hovered file if none selected) from the active pane's active tab to the other pane's active tab's current working directory.  This operation SHALL NOT use the yank register.  After the operation, the selection state SHALL be cleared.

#### Scenario: Move single file to other pane
- **WHEN** the user has no files selected and the cursor is on "file.txt" in the left pane, and the right pane is in /tmp
- **THEN** pressing F6 moves file.txt to /tmp/file.txt

#### Scenario: Move multiple selected files to other pane
- **WHEN** the user has 3 files selected in the left pane, and the right pane is in /home/user/archive
- **THEN** pressing F6 moves all 3 files to /home/user/archive/

#### Scenario: Move files to other pane's active tab
- **WHEN** the user selects files in the left pane's active tab and presses F6
- **THEN** the files are moved to the right pane's active tab's current working directory

#### Scenario: Move when other pane has multiple tabs
- **WHEN** the left pane has 2 tabs with tab 1 active, and the right pane has 3 tabs with tab 3 active, and the user moves from the left pane
- **THEN** the files are moved to the right pane's tab 3's current working directory

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

### Requirement: Force copy to other pane (Shift+F5)
The system SHALL provide a force variant of `copy_to` that overwrites existing files at the destination.  When a file with the same name exists in the destination, it SHALL be trashed and replaced by the source file.  This is equivalent to `copy_to --force`.

#### Scenario: Force copy overwrites existing file
- **WHEN** the user has "file.txt" selected in the left pane, and the right pane's directory already contains "file.txt", and the user executes `copy_to --force`
- **THEN** the existing "file.txt" in the right pane's directory is trashed, and the source "file.txt" is copied to the destination

#### Scenario: Force copy with no conflict
- **WHEN** the user has "new_file.txt" selected and the destination has no file with that name, and the user executes `copy_to --force`
- **THEN** the file is copied normally, identical to non-force copy

#### Scenario: Force copy records undo with overwritten entry
- **WHEN** a force copy overwrites an existing file
- **THEN** the undo entry records both the copy pair and the overwritten file's trash path, allowing undo to restore the original file

### Requirement: Force move to other pane (Shift+F6)
The system SHALL provide a force variant of `move_to` that overwrites existing files at the destination.  When a file with the same name exists in the destination, it SHALL be trashed and replaced by the source file.  This is equivalent to `move_to --force`.

#### Scenario: Force move overwrites existing file
- **WHEN** the user has "data.csv" selected in the left pane, and the right pane's directory already contains "data.csv", and the user executes `move_to --force`
- **THEN** the existing "data.csv" in the right pane's directory is trashed, and the source "data.csv" is moved to the destination

#### Scenario: Force move with no conflict
- **WHEN** the user has "report.pdf" selected and the destination has no file with that name, and the user executes `move_to --force`
- **THEN** the file is moved normally, identical to non-force move

#### Scenario: Force move records undo with overwritten entry
- **WHEN** a force move overwrites an existing file
- **THEN** the undo entry records both the move pair and the overwritten file's trash path, allowing undo to restore the original file

### Requirement: Existing yank/paste cross-pane compatibility
The existing yank (`y`) and paste (`p`) workflow SHALL continue to work across panes.  When the user yanks in one pane, switches to the other pane, and pastes, the paste destination SHALL be the active pane's current working directory.

#### Scenario: Yank in left, paste in right
- **WHEN** the user yanks files in the left pane, switches to the right pane, and pastes
- **THEN** files are pasted to the right pane's current directory
