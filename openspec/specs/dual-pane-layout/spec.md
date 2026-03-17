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
The system SHALL render two panes horizontally with a vertical separator between them when in dual-pane mode.  When in single-pane mode, the system SHALL render only the active pane at full width using the user's configured ratio (default `[1, 4, 3]`).  Each pane in dual-pane mode SHALL use ratio `[1, 2, 0]` by default (parent + current, no preview).  When preview_pane mode is enabled, each pane SHALL use ratio `[0, 1, 1]` (current + preview, no parent).  The two panes SHALL each occupy approximately 50% of the available width, minus the separator.

#### Scenario: Normal terminal width in dual-pane mode
- **WHEN** the terminal has sufficient width (>= 80 columns) and the system is in dual-pane mode with preview_pane disabled
- **THEN** two panes are rendered side-by-side, each showing parent and current columns, separated by a vertical line

#### Scenario: Dual-pane preview mode rendering
- **WHEN** the system is in dual-pane mode with preview_pane enabled
- **THEN** two panes are rendered side-by-side, each showing current and preview columns (no parent column), separated by a vertical line

#### Scenario: Single-pane mode rendering
- **WHEN** the system is in single-pane mode
- **THEN** only the active pane is rendered at full width with parent, current, and preview columns

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
The Header SHALL display information for the active pane only.  The Status bar SHALL display information for the active pane only.

#### Scenario: Header shows active pane path
- **WHEN** the left pane is active and browsing /home/user/docs
- **THEN** the header displays the path /home/user/docs

#### Scenario: Switching active pane updates header
- **WHEN** the user switches from left pane (in /home/user/docs) to right pane (in /tmp)
- **THEN** the header updates to display /tmp
