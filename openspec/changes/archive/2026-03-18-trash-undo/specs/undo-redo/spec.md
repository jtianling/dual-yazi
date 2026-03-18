## ADDED Requirements

### Requirement: Undo restores trashed files to original location
The system SHALL restore trashed files to their original location when undoing a Trash operation.

#### Scenario: Undo a trash operation
- **WHEN** user presses `u` and the top undo entry is Trash { pairs }
- **THEN** the system SHALL move each file from its `trash_path` back to its `original_path` using `provider::rename()`
- **AND** the entry SHALL be moved to the redo stack
- **AND** appropriate FilesOp events SHALL be emitted to update the UI

#### Scenario: Undo trash when file no longer in trash
- **WHEN** user presses `u` and a file in the Trash entry's pairs no longer exists at its `trash_path`
- **THEN** the system SHALL skip that file silently
- **AND** the entry SHALL still be moved to the redo stack

### Requirement: Redo re-trashes previously restored files
The system SHALL re-trash files when redoing a Trash operation, obtaining new trash paths.

#### Scenario: Redo a trash operation
- **WHEN** user presses `<C-r>` and the top redo entry is Trash { pairs }
- **THEN** the system SHALL call `provider::trash()` for each `original_path` in the pairs
- **AND** the undo entry pushed back onto the undo stack SHALL contain the updated trash paths from the new trash operation

#### Scenario: Redo trash when file no longer exists at original path
- **WHEN** user presses `<C-r>` and a file in the Trash entry no longer exists at its `original_path`
- **THEN** the system SHALL skip that file silently

## MODIFIED Requirements

### Requirement: Non-undoable operations are not recorded
The system SHALL NOT record undo entries for permanent delete, shell commands, bulk rename, or plugin operations.

#### Scenario: Permanent delete is not recorded
- **WHEN** user permanently deletes a file
- **THEN** no undo entry SHALL be created
