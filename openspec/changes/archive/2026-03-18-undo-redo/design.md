## Context

dual-yazi is a fork of yazi file manager with dual-pane support. File operations (rename, create, copy, move, delete) are dispatched through an Actor system and executed asynchronously via a Scheduler. The default keymap already binds `u` → `undo` and `<C-r>` → `redo`, but no Actor implementation exists — they silently no-op.

Key architectural facts:
- Actors implement `fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data>`
- `Ctx` provides access to `core.mgr` (manager state) and `core.tasks` (scheduler)
- Rename and Create are async but self-contained (`tokio::spawn` with `.await`)
- Paste/Remove submit work to the Scheduler and return immediately
- `Yanked` buffer (copy/cut clipboard) is a `Mgr`-level field, same level where `UndoManager` will live

## Goals / Non-Goals

**Goals:**
- Implement undo/redo for rename, create, copy (paste), and move (cut+paste, copy_to, move_to)
- Follow vim's undo model: one user action = one undo entry
- Integrate naturally with existing Actor/Executor architecture
- Minimal code changes to existing actors (one push call per operation)

**Non-Goals:**
- Trash restore (future work — needs platform-specific APIs)
- Permanent delete undo (impossible without backup)
- Persisting undo history across app restarts
- Undo for shell commands, bulk rename, or plugin operations
- Undo tree (vim has a tree; we use a simple linear stack)

## Decisions

### 1. UndoManager lives in Mgr

**Decision**: Add `UndoManager` as a field on `Mgr`, alongside `yanked`.

**Why**: Undo is a file-manager-level concern, not per-tab or per-pane. Same granularity as `Yanked`. Accessed via `cx.mgr.undo`.

**Alternative**: Separate `yazi-undo` crate — rejected because the feature isn't large enough to warrant its own crate.

### 2. Optimistic recording at Actor layer

**Decision**: Record undo entries in the Actor's `act()` method at the moment the operation is dispatched, before async completion.

**Why**:
- Rename/Create run in `tokio::spawn` and can't easily write back to `Ctx`
- Paste/Remove submit to Scheduler and return immediately
- Adding completion callbacks would require significant Scheduler changes
- If an operation fails, undo will encounter "file not found" and silently skip — acceptable behavior

**Alternative**: Record after Scheduler confirms success — rejected because it requires adding callback channels through the Scheduler pipeline, significantly more complex for marginal benefit.

### 3. Undo/Redo actions execute synchronously via tokio::spawn

**Decision**: `Undo::act()` and `Redo::act()` pop from the stack and spawn a tokio task that executes the reverse operation using existing `provider::*` functions directly (not through the Scheduler).

**Why**:
- Reverse operations are typically simple (one rename, delete a created file, move files back)
- Going through the Scheduler would create circular undo recording
- Direct `provider::*` calls are already used by Rename and Create actors

### 4. UndoOp captures all data needed for reversal

**Decision**: Each `UndoOp` variant stores enough information to execute both the reverse and the re-do without querying external state.

**Why**: By the time undo runs, the filesystem state may have changed. Having self-contained entries means we attempt the reversal regardless and handle errors gracefully.

### 5. Stack size limit of 20

**Decision**: Hard cap at 20 entries. Oldest entries are dropped when the limit is exceeded.

**Why**: User preference. File operations are heavyweight enough that 20 provides ample history.

## Risks / Trade-offs

**[Optimistic recording may create phantom undo entries]** → If an operation fails (e.g., permission denied on copy), the undo entry still exists. Mitigation: undo of a failed operation is a no-op (files don't exist to reverse). User sees a brief error notification, which is acceptable.

**[Race condition between undo and in-flight operations]** → User could press `u` while a large copy is still running. Mitigation: For this version, undo will attempt the reverse immediately. If files are still being written, the reverse may partially fail. This is an edge case; users rarely undo operations that are visibly still running.

**[Undo doesn't go through Scheduler, so no progress tracking]** → Reverse operations won't appear in the task manager. Mitigation: Reverse operations are typically fast (rename, delete). If needed later, we can route through Scheduler.

**[No FilesOp emission from undo actions]** → Need to manually emit `FilesOp` events after undo so the UI updates. This requires care to emit the correct Creating/Deleting/Upserting variants.
