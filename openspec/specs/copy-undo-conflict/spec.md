## Purpose

Handles copy/cut undo behavior in file naming conflict and force overwrite scenarios, ensuring undo entries record actual destination paths and that force-overwritten files can be restored.

## Requirements

### Requirement: Copy undo entry records actual destination path after rename
When a non-force copy/cut operation creates renamed files due to naming conflicts, the undo entry SHALL record the actual destination paths (after `unique_file` rename) instead of the pre-rename paths.

#### Scenario: Copy with naming conflict records renamed path
- **WHEN** user copies `a.txt` to a directory that already contains `a.txt` (non-force paste)
- **AND** the scheduler renames the destination to `a_1.txt` via `unique_file()`
- **THEN** the undo entry's pairs SHALL contain `(source/a.txt, dest/a_1.txt)` as the actual destination

#### Scenario: Copy without naming conflict records original path
- **WHEN** user copies `a.txt` to a directory that does not contain `a.txt`
- **THEN** the undo entry's pairs SHALL contain `(source/a.txt, dest/a.txt)` as the destination

#### Scenario: Undo of renamed copy deletes the renamed file
- **WHEN** user undoes a copy whose undo entry contains `(source/a.txt, dest/a_1.txt)`
- **THEN** the system SHALL delete `dest/a_1.txt`
- **AND** the original `dest/a.txt` SHALL remain intact

#### Scenario: Cut with naming conflict records renamed path
- **WHEN** user cuts `a.txt` to a directory that already contains `a.txt` (non-force paste)
- **AND** the scheduler renames the destination to `a_1.txt`
- **THEN** the undo entry's pairs SHALL contain `(source/a.txt, dest/a_1.txt)`

#### Scenario: Undo of renamed cut moves renamed file back to source
- **WHEN** user undoes a cut whose undo entry contains `(source/a.txt, dest/a_1.txt)`
- **THEN** the system SHALL rename `dest/a_1.txt` back to `source/a.txt`

### Requirement: Undo entry pairs populated via hook callback
The paste actors (paste, copy_to, move_to) SHALL push Copy/Move undo entries with empty pairs. The actual pairs SHALL be populated incrementally by scheduler hooks after each file operation completes.

#### Scenario: Paste pushes empty Copy entry then hook populates it
- **WHEN** user executes paste (copy mode) for files `a.txt` and `b.txt`
- **THEN** an `UndoOp::Copy { pairs: [], overwritten: [] }` entry SHALL be pushed immediately
- **AND** as each file copy completes, the hook SHALL call `push_copy_pair(from, actual_to)` to append the pair to the latest Copy entry

#### Scenario: Paste pushes empty Move entry then hook populates it
- **WHEN** user executes paste (cut mode) for files `a.txt` and `b.txt`
- **THEN** an `UndoOp::Move { pairs: [], overwritten: [] }` entry SHALL be pushed immediately
- **AND** as each file cut completes, the hook SHALL call `push_move_pair(from, actual_to)` to append the pair

#### Scenario: copy_to pushes empty Copy entry then hook populates it
- **WHEN** user executes copy_to (F5) for selected files
- **THEN** an `UndoOp::Copy { pairs: [], overwritten: [] }` entry SHALL be pushed immediately
- **AND** hooks SHALL populate actual pairs as copies complete

#### Scenario: move_to pushes empty Move entry then hook populates it
- **WHEN** user executes move_to (F6) for selected files
- **THEN** an `UndoOp::Move { pairs: [], overwritten: [] }` entry SHALL be pushed immediately
- **AND** hooks SHALL populate actual pairs as cuts complete

### Requirement: Force overwrite trashes original before copying
When a force copy/cut operation (`P` or `--force`) overwrites an existing file, the system SHALL trash the existing file before performing the overwrite, enabling undo to restore the original.

#### Scenario: Force copy trashes existing file before overwrite
- **WHEN** user force-pastes `a.txt` to a directory containing `a.txt`
- **THEN** the scheduler SHALL trash the existing `dest/a.txt` before copying the new one
- **AND** the trash path SHALL be recorded in the undo entry's `overwritten` field as `(dest/a.txt, trash_path)`

#### Scenario: Force copy with no existing file does not trash
- **WHEN** user force-pastes `a.txt` to a directory that does not contain `a.txt`
- **THEN** no trash operation SHALL occur
- **AND** the `overwritten` field SHALL remain empty

#### Scenario: Undo of force copy restores original from trash
- **WHEN** user undoes a force copy with `overwritten: [(dest/a.txt, trash_path)]` and `pairs: [(source/a.txt, dest/a.txt)]`
- **THEN** the system SHALL first delete `dest/a.txt` (the new copy)
- **AND** then restore the original by renaming `trash_path` back to `dest/a.txt`
- **AND** emit appropriate FilesOp events to update the UI

#### Scenario: Force cut trashes existing file before overwrite
- **WHEN** user force-pastes (cut mode) `a.txt` to a directory containing `a.txt`
- **THEN** the scheduler SHALL trash the existing `dest/a.txt` before moving the source
- **AND** the trash path SHALL be recorded in the undo entry's `overwritten` field

#### Scenario: Undo of force cut restores both source and overwritten original
- **WHEN** user undoes a force cut with `overwritten: [(dest/a.txt, trash_path)]` and `pairs: [(source/a.txt, dest/a.txt)]`
- **THEN** the system SHALL move `dest/a.txt` back to `source/a.txt`
- **AND** then restore the overwritten original by renaming `trash_path` back to `dest/a.txt`

#### Scenario: Trash before overwrite fails gracefully
- **WHEN** `provider::trash()` fails for the existing file (e.g., cross-filesystem)
- **THEN** the scheduler SHALL proceed with the force copy/cut as before (overwrite without backup)
- **AND** the `overwritten` field SHALL not contain an entry for this file
