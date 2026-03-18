## Context

The undo/redo system records file operations (rename, create, copy, move) into an undo stack at the Mgr level. The recording happens synchronously in the actor layer before the async scheduler processes the actual file operation ("optimistic recording"). Trash operations were excluded because the trash destination path is non-deterministic — the OS may rename files to avoid collisions in the trash directory.

The `trash` crate (v5.2.5) is used for trash operations. On macOS, it calls `NSFileManager.trashItemAtURL:resultingItemURL:error:`, which accepts an out-parameter `resultingItemURL` that returns the actual path of the trashed file. The current code passes `None` for this parameter, discarding the information.

The scheduler processes each file as an independent task. Trash goes through: `RemoveDo` actor → `scheduler.file_trash(target)` → `File::trash_do()` → `provider::trash()`. After completion, a Hook fires which notifies the watcher and publishes DDS events.

## Goals / Non-Goals

**Goals:**
- Enable undo for trash operations, restoring files from trash to their original location
- Enable redo for trash operations, re-trashing previously restored files
- Merge multiple files trashed in a single user action into one undo entry
- Keep the trash execution path through the existing scheduler pipeline unchanged
- Capture the actual trash destination path on macOS via `resultingItemURL`

**Non-Goals:**
- Undo for permanent delete (destructive, unrecoverable)
- Undo for SFTP trash (unsupported by the protocol)
- Cross-session undo persistence (undo stack is in-memory only)
- Undo for shell commands, bulk rename, or plugin operations

## Decisions

### Decision 1: Thread trash_path through scheduler via `reduce()` injection into Hook

**Choice**: Modify `provider::trash()` to return the trash path. Carry it through `FileOutTrash::Succ(UrlBuf)`. In `FileOutTrash::reduce()`, inject the path directly into the `HookInTrash` via `task.hook`.

**Why**: The `reduce()` method already receives `&mut Task`, which contains `hook: Option<HookIn>`. This avoids changes to `FileProgTrash` (which derives `Copy` and would break the entire `TaskProg` hierarchy), avoids changes to the `Task` struct, and requires zero changes to `runner.rs`.

**Alternatives considered**:
- *Store in FileProgTrash*: Requires adding `UrlBuf` to prog, breaking `Copy` derive across `TaskProg`, `FileProgTrash`, and all serialization — too invasive.
- *Store in Task struct*: Adds a trash-specific field to the generic `Task` — pollutes the common struct.
- *Bypass scheduler entirely*: Loses scheduler features (task management, progress UI). User explicitly rejected this.

### Decision 2: Merge consecutive trash pairs in UndoManager

**Choice**: Add `push_trash_pair(original, trash_path)` to `UndoManager`. If the top of the undo stack is already a `Trash` entry, append the pair to it. Otherwise, create a new `Trash` entry.

**Why**: When the user trashes 5 files at once, the scheduler fires 5 independent hooks. Without merging, this would create 5 separate undo entries, requiring 5 presses of `u` to restore. The merge produces one entry for the whole batch. Since hooks fire in rapid succession from the same scheduler batch, consecutive Trash pairs reliably belong to the same user action.

**Alternatives considered**:
- *One undo entry per file*: Simple but terrible UX — trashing 10 files uses half the undo stack.
- *Batch ID coordination*: Pass a batch ID from RemoveDo through the scheduler to hooks. Clean but invasive — requires threading an ID through every layer.
- *Optimistic pre-recording*: Record the undo entry before scheduling (like copy/move). Not possible — trash destination path is unknown at that point.

### Decision 3: Capture `resultingItemURL` on macOS by bypassing the trash crate

**Choice**: In the macOS local provider's `trash()` implementation, replace the `trash` crate call with a direct `NSFileManager.trashItemAtURL:resultingItemURL:error:` call using `objc2_foundation` (already a transitive dependency). Pass `Some(&mut result_url)` for the `resultingItemURL` parameter to capture the actual trash path.

**Why**: The `trash` crate's `TrashContext::delete()` does not expose the resulting URL — it calls `trashItemAtURL` with `None` for the out-parameter internally. Forking the crate is heavyweight for a one-line change. Direct NSFileManager usage gives us the exact trash path with minimal code.

**Alternatives considered**:
- *Fork the trash crate*: Maintenance burden for a single API change.
- *Predict the trash path (~/.Trash/filename)*: Fragile — macOS renames on collision (e.g., `foo.txt` → `foo 2.txt`).
- *Search trash at undo time*: Unreliable — multiple files with same name, timing issues.

### Decision 4: Redo trash re-executes the trash operation

**Choice**: `redo_trash` calls `provider::trash()` again for each file, obtaining new trash paths, and pushes a fresh `Trash` undo entry with the updated pairs.

**Why**: Unlike redo for rename/move (which are deterministic rename operations), redo trash must go through the OS trash mechanism to maintain correct trash metadata. The new trash path may differ from the original, so the undo entry must be updated.

### Decision 5: Linux trash path construction

**Choice**: On Linux (Freedesktop), the trash path is deterministic: `$trash_dir/files/$filename`. After calling `trash::delete()`, construct the path from the known trash directory and the original filename. Handle potential renaming (`.2`, `.3` suffixes) by checking which file exists.

**Why**: The `trash` crate on Linux follows the Freedesktop Trash specification. The trashed file always ends up in the `files/` subdirectory of the trash folder. Unlike macOS, we can use the trash crate's existing `delete()` and then locate the result.

## Risks / Trade-offs

- **[Merge window race]** → If the user performs another undoable operation (e.g., rename) between trash hook callbacks, `push_trash_pair` will correctly start a new entry since the stack top won't be a `Trash` entry. This splits one batch into two entries — acceptable degradation, not data loss.

- **[macOS API stability]** → Direct `NSFileManager` usage via `objc2_foundation` ties us to the Objective-C bridge. Mitigation: `objc2_foundation` is already a transitive dependency of the `trash` crate itself, so no new dependency risk.

- **[Linux path prediction]** → Freedesktop trash may use different trash directories for different mount points. Mitigation: The `trash` crate handles mount-point-aware trashing; we only need to discover the resulting path post-deletion, which follows the spec.

- **[Redo trash path divergence]** → After undo+redo, the trash path changes. This is correct behavior — each trash operation gets its own trash metadata. The undo entry is updated accordingly.
