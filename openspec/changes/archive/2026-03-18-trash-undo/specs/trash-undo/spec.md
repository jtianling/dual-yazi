## ADDED Requirements

### Requirement: Trash provider returns the trash destination path
The `provider::trash()` function SHALL return `io::Result<UrlBuf>` containing the actual path of the file in the trash directory. On macOS, this SHALL be obtained via `NSFileManager.trashItemAtURL:resultingItemURL:error:`. On Linux, this SHALL be constructed from the Freedesktop trash directory after deletion. On platforms where trash path capture is unsupported (SFTP, Android), the function SHALL return an error.

#### Scenario: macOS trash captures resultingItemURL
- **WHEN** a local file is trashed on macOS
- **THEN** the provider SHALL call `trashItemAtURL:resultingItemURL:error:` with a non-nil `resultingItemURL` out-parameter
- **AND** SHALL return the resulting URL as a `UrlBuf`

#### Scenario: macOS trash with name collision
- **WHEN** a file "foo.txt" is trashed on macOS and `~/.Trash/foo.txt` already exists
- **THEN** the provider SHALL return the actual renamed path (e.g., `~/.Trash/foo 2.txt`) as captured from `resultingItemURL`

#### Scenario: Linux trash path construction
- **WHEN** a local file is trashed on Linux
- **THEN** the provider SHALL call `trash::delete()` and then locate the resulting file in the Freedesktop trash `files/` directory
- **AND** SHALL return the trash path as a `UrlBuf`

#### Scenario: SFTP trash path is unsupported
- **WHEN** a file on an SFTP remote is trashed
- **THEN** the provider SHALL return an `io::ErrorKind::Unsupported` error
- **AND** no undo entry SHALL be recorded

### Requirement: Scheduler threads trash path through Hook
The scheduler SHALL propagate the trash destination path from the provider through `FileOutTrash` to `HookInTrash`. The `FileOutTrash::reduce()` method SHALL inject the trash path into the `HookInTrash` stored in `task.hook`.

#### Scenario: Successful trash threads path to Hook
- **WHEN** `provider::trash()` succeeds and returns a trash path
- **THEN** `FileOutTrash::Succ(trash_path)` SHALL carry the path
- **AND** `reduce()` SHALL set `HookInTrash.trash_path` to `Some(trash_path)` via `task.hook`

#### Scenario: Failed trash does not thread path
- **WHEN** `provider::trash()` fails
- **THEN** `FileOutTrash::Fail` SHALL be emitted
- **AND** no trash path SHALL be injected into the hook

### Requirement: Hook pushes trash undo entry via MgrProxy
When a trash operation completes successfully, `Hook::trash()` SHALL call `MgrProxy::undo_push_trash_pair(target, trash_path)` to record the operation in the undo stack.

#### Scenario: Hook records trash undo after successful trash
- **WHEN** `Hook::trash()` fires with `intact == true` and `trash_path` is `Some`
- **THEN** it SHALL call `MgrProxy::undo_push_trash_pair(target, trash_path)`

#### Scenario: Hook skips undo for canceled or failed trash
- **WHEN** `Hook::trash()` fires with `intact == false` or `trash_path` is `None`
- **THEN** no undo entry SHALL be recorded

### Requirement: UndoManager merges consecutive trash pairs
`UndoManager::push_trash_pair()` SHALL merge consecutive trash pairs into a single `UndoOp::Trash` entry. If the top of the undo stack is a `Trash` entry, the pair SHALL be appended to its `pairs` list. Otherwise, a new `Trash` entry SHALL be created.

#### Scenario: First trash pair creates new entry
- **WHEN** `push_trash_pair(original, trash_path)` is called and the undo stack top is not a Trash entry
- **THEN** a new `UndoOp::Trash { pairs: [(original, trash_path)] }` SHALL be pushed
- **AND** the redo stack SHALL be cleared

#### Scenario: Subsequent trash pair merges into existing entry
- **WHEN** `push_trash_pair(original, trash_path)` is called and the undo stack top is a Trash entry
- **THEN** the pair SHALL be appended to the existing entry's `pairs` list
- **AND** the redo stack SHALL NOT be cleared again

#### Scenario: Non-trash operation breaks the merge
- **WHEN** a rename operation is recorded between two trash operations
- **THEN** each trash operation SHALL have its own separate undo entry
