# Capability: Dual Pane Layout

## Purpose

Defines the visual layout and rendering behavior for the dual-pane file manager mode, including pane initialization, separator, active pane indicators, and header/status bar adaptation.

## Requirements

### Requirement: Dual pane initialization
The system SHALL initialize with exactly two panes on startup.  The left pane (index 0) SHALL be the active pane by default.  Both panes SHALL start in the user's current working directory.

#### Scenario: Application startup
- **WHEN** the application starts
- **THEN** two panes are displayed side-by-side, left pane is active, both showing the current working directory

### Requirement: Dual pane layout rendering
The system SHALL render two panes horizontally with a vertical separator between them when in dual-pane mode. Each pane SHALL be a self-contained vertical layout consisting of a Header row, the Tab content area, and a Status bar row. When in single-pane mode, the system SHALL render only the active pane at full width using the user's configured ratio (default `[1, 4, 3]`) with root-level Header and Status. Each pane in dual-pane mode SHALL use ratio `[1, 2, 0]` by default (parent + current, no preview). When preview_pane mode is enabled, each pane SHALL use ratio `[0, 1, 1]` (current + preview, no parent). The two panes SHALL each occupy approximately 50% of the available width, minus the separator. The root layout in dual-pane mode SHALL allocate the full screen height to the DualPane component (no root-level Header or Status rows).

#### Scenario: Normal terminal width in dual-pane mode
- **WHEN** the terminal has sufficient width (>= 80 columns) and the system is in dual-pane mode with preview_pane disabled
- **THEN** two panes are rendered side-by-side, each with its own header, parent and current columns, and status bar, separated by a vertical line

#### Scenario: Dual-pane preview mode rendering
- **WHEN** the system is in dual-pane mode with preview_pane enabled
- **THEN** two panes are rendered side-by-side, each with its own header, current and preview columns, and status bar, separated by a vertical line

#### Scenario: Single-pane mode rendering
- **WHEN** the system is in single-pane mode
- **THEN** only the active pane is rendered at full width with root-level header, parent, current, and preview columns, and root-level status bar

#### Scenario: Ratio configuration
- **WHEN** the user configures a custom ratio (e.g., `[1, 2, 1]`)
- **THEN** single-pane mode SHALL use the configured ratio; dual-pane default mode SHALL use `[1, 2, 0]`; dual-pane preview mode SHALL use `[0, 1, 1]`

### Requirement: Active pane visual indicator
The system SHALL visually distinguish the active pane from the inactive pane.  The inactive pane SHALL have a dimmed or reduced contrast appearance compared to the active pane.

#### Scenario: Left pane active
- **WHEN** the left pane is active
- **THEN** the left pane renders with normal brightness and the right pane renders with reduced contrast

#### Scenario: Right pane active
- **WHEN** the right pane is active
- **THEN** the right pane renders with normal brightness and the left pane renders with reduced contrast

### Requirement: Separator rendering
The system SHALL render a vertical separator line between the two panes.  The separator SHALL be a single character wide.

#### Scenario: Separator display
- **WHEN** two panes are displayed
- **THEN** a vertical line separator (│) is rendered between them

### Requirement: Tab bar removal
The system SHALL NOT display a tab bar.  The area previously used by the tab bar SHALL be allocated to the pane content area, whether in dual-pane or single-pane mode.

#### Scenario: No tab bar displayed
- **WHEN** the application is running in either mode
- **THEN** no tab bar is visible, and the full vertical space (minus header and status) is used for pane content

### Requirement: Header and status bar adaptation
In dual-pane mode, each pane SHALL have its own independent Header and Status bar. The Header SHALL display the directory path and file count for that specific pane. The Status bar SHALL display the mode, file name, permissions, and position for that specific pane. In single-pane mode, a single Header and Status bar SHALL be rendered at the root level for the active pane, as before.

#### Scenario: Dual-pane mode shows per-pane headers
- **WHEN** the system is in dual-pane mode with left pane browsing /home/user/docs and right pane browsing /tmp
- **THEN** the left pane displays its own header showing /home/user/docs and the right pane displays its own header showing /tmp

#### Scenario: Dual-pane mode shows per-pane status bars
- **WHEN** the system is in dual-pane mode with left pane hovering file "readme.md" and right pane hovering file "config.toml"
- **THEN** the left pane displays its own status bar with mode, size, and name for "readme.md" and the right pane displays its own status bar with mode, size, and name for "config.toml"

#### Scenario: Dual-pane mode shows per-pane file attributes
- **WHEN** the system is in dual-pane mode with left pane hovering a file with permissions -rw-r--r-- and right pane hovering a file with permissions -rwxr-xr-x
- **THEN** the left pane's status bar shows -rw-r--r-- and the right pane's status bar shows -rwxr-xr-x

#### Scenario: Single-pane mode unchanged
- **WHEN** the system is in single-pane mode
- **THEN** a single Header and Status bar are rendered at the root level showing the active pane's information

#### Scenario: Switching active pane updates visual indicator only
- **WHEN** the user switches the active pane in dual-pane mode
- **THEN** both panes continue to show their own Header and Status bars, and the active pane indicator (brightness) updates accordingly
