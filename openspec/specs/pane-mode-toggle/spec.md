# Capability: Pane Mode Toggle

## Purpose

Defines the ability to toggle between dual-pane and single-pane layout modes, preserving inactive pane state and restoring the full preview experience in single-pane mode.

## Requirements

### Requirement: Toggle between dual-pane and single-pane mode
The system SHALL provide a `pane_only` action that toggles the layout between dual-pane mode and single-pane mode.  In dual-pane mode, both panes are rendered side-by-side.  In single-pane mode, only the active pane is rendered at full width with preview restored.

#### Scenario: Switch from dual-pane to single-pane
- **WHEN** the system is in dual-pane mode and the user triggers pane_only
- **THEN** the layout switches to single-pane mode, showing only the active pane at full width with the configured ratio (including preview)

#### Scenario: Switch from single-pane to dual-pane
- **WHEN** the system is in single-pane mode and the user triggers pane_only
- **THEN** the layout switches back to dual-pane mode, showing both panes side-by-side with dual-pane ratio (no preview)

#### Scenario: Default startup mode
- **WHEN** the application starts
- **THEN** the system SHALL be in dual-pane mode

### Requirement: Inactive pane state preservation
The system SHALL preserve the inactive pane's complete state (directory, cursor position, selection, history) when switching to single-pane mode.  When switching back to dual-pane mode, the inactive pane SHALL render with its preserved state.

#### Scenario: Preserve directory state
- **WHEN** the right pane is in /tmp with 3 files selected, the left pane is active, and the user switches to single-pane mode
- **THEN** the right pane is hidden but its state (/tmp directory, 3 selected files) is preserved

#### Scenario: Restore state on return to dual-pane
- **WHEN** the system is in single-pane mode and the user switches back to dual-pane mode
- **THEN** both panes render with their preserved states intact

### Requirement: Single-pane mode uses configured ratio with preview
In single-pane mode, the active pane SHALL use the user's configured `mgr.ratio` value (default `[1, 4, 3]`), which includes the preview column.  This restores the original yazi miller columns experience.

#### Scenario: Preview visible in single-pane mode
- **WHEN** the system is in single-pane mode with default ratio configuration
- **THEN** the active pane displays parent, current, and preview columns using ratio [1, 4, 3]

#### Scenario: Custom ratio in single-pane mode
- **WHEN** the user has configured ratio as [1, 3, 2] and switches to single-pane mode
- **THEN** the active pane uses ratio [1, 3, 2]

### Requirement: Ctrl-w o keybinding
The system SHALL bind `Ctrl-w o` to the `pane_only` action.

#### Scenario: Ctrl-w o triggers toggle
- **WHEN** the user presses Ctrl-w followed by o
- **THEN** the pane_only action is executed, toggling the layout mode

### Requirement: Cross-pane operations work in single-pane mode
The `pane_switch`, `copy_to`, and `move_to` actions SHALL continue to function in single-pane mode.  `pane_switch` SHALL switch the active tab index (changing which pane is rendered).  `copy_to` and `move_to` SHALL use the hidden pane's cwd as destination.

#### Scenario: Pane switch in single-pane mode
- **WHEN** the system is in single-pane mode with the left pane active (in /home) and the hidden right pane is in /tmp
- **THEN** pressing Tab switches to the right pane, and the view now shows /tmp with preview

#### Scenario: Copy to hidden pane
- **WHEN** the system is in single-pane mode, the active pane has file.txt selected, and the hidden pane is in /backup
- **THEN** pressing F5 copies file.txt to /backup/file.txt

### Requirement: single_pane state exposed to Lua
The system SHALL expose the current mode state to Lua as `cx.tabs.single_pane` (boolean).  Lua UI components SHALL use this value to decide whether to render DualPane or single Tab layout.

#### Scenario: Lua reads single_pane state
- **WHEN** Lua rendering occurs
- **THEN** `cx.tabs.single_pane` returns `false` in dual-pane mode and `true` in single-pane mode
