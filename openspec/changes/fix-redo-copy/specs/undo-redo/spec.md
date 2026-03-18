## MODIFIED Requirements

### Requirement: Undo stack records file operations
The system SHALL maintain an undo stack (max 20 entries) at the Mgr level that records completed file operations. When a new operation is recorded, the redo stack SHALL be cleared. When the stack exceeds 20 entries, the oldest entry SHALL be dropped.

#### Scenario: Recording a rename operation
- **WHEN** user renames a file from "old.txt" to "new.txt"
- **THEN** an undo entry of type Rename { old, new } SHALL be pushed onto the undo stack

#### Scenario: Recording a create operation
- **WHEN** user creates a new file or directory
- **THEN** an undo entry of type Create { target, is_dir } SHALL be pushed onto the undo stack

#### Scenario: Recording a copy-paste operation
- **WHEN** user pastes yanked files (copy mode) into a destination directory
- **THEN** an undo entry of type Copy { pairs } SHALL be pushed, where pairs maps each source path to its destination path

#### Scenario: Recording a cut-paste operation
- **WHEN** user pastes yanked files (cut mode) into a destination directory
- **THEN** an undo entry of type Move { pairs } SHALL be pushed, where pairs maps each source path to its destination path

#### Scenario: Recording a copy_to cross-pane operation
- **WHEN** user executes copy_to (copying selected files to the other pane)
- **THEN** an undo entry of type Copy { pairs } SHALL be pushed with source-to-destination path pairs

#### Scenario: Recording a move_to cross-pane operation
- **WHEN** user executes move_to (moving selected files to the other pane)
- **THEN** an undo entry of type Move { pairs } SHALL be pushed with source-to-destination path pairs

#### Scenario: New operation clears redo stack
- **WHEN** user performs any undoable file operation after having undone previous operations
- **THEN** the redo stack SHALL be cleared

#### Scenario: Stack overflow drops oldest entry
- **WHEN** the undo stack has 20 entries and a new operation is recorded
- **THEN** the oldest (bottom) entry SHALL be removed before the new entry is pushed

### Requirement: Redo re-applies the most recently undone operation
The system SHALL re-apply the most recently undone operation when the user presses `<C-r>` in the Mgr layer. The re-applied entry SHALL be moved back to the undo stack.

#### Scenario: Redo a rename
- **WHEN** user presses `<C-r>` and the top redo entry is Rename { old, new }
- **THEN** the system SHALL rename the file from `old` to `new`
- **AND** the entry SHALL be moved back to the undo stack

#### Scenario: Redo a create
- **WHEN** user presses `<C-r>` and the top redo entry is Create { target, is_dir }
- **THEN** the system SHALL recreate the file or directory
- **AND** the entry SHALL be moved back to the undo stack

#### Scenario: Redo a copy
- **WHEN** user presses `<C-r>` and the top redo entry is Copy { pairs }
- **THEN** the system SHALL re-copy each source to its destination via the Scheduler (with progress tracking and cancellation support)
- **AND** the entry SHALL be moved back to the undo stack

#### Scenario: Redo a move
- **WHEN** user presses `<C-r>` and the top redo entry is Move { pairs }
- **THEN** the system SHALL move each file from its source to its destination again
- **AND** the entry SHALL be moved back to the undo stack

#### Scenario: Redo with empty stack
- **WHEN** user presses `<C-r>` and the redo stack is empty
- **THEN** the system SHALL do nothing
