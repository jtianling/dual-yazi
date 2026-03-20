## ADDED Requirements

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
