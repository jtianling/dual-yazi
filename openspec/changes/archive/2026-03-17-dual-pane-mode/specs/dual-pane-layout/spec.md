## ADDED Requirements

### Requirement: Dual pane initialization
The system SHALL initialize with exactly two panes on startup.  The left pane (index 0) SHALL be the active pane by default.  Both panes SHALL start in the user's current working directory.

#### Scenario: Application startup
- **WHEN** the application starts
- **THEN** two panes are displayed side-by-side, left pane is active, both showing the current working directory

### Requirement: Dual pane layout rendering
The system SHALL render two panes horizontally with a vertical separator between them.  Each pane SHALL use the miller columns layout with ratio `[1, 2, 0]` by default (parent + current, no preview).  The two panes SHALL each occupy approximately 50% of the available width, minus the separator.

#### Scenario: Normal terminal width
- **WHEN** the terminal has sufficient width (>= 80 columns)
- **THEN** two panes are rendered side-by-side, each showing parent and current columns, separated by a vertical line

#### Scenario: Ratio configuration
- **WHEN** the user configures a custom ratio (e.g., `[1, 2, 1]`)
- **THEN** each pane SHALL use the configured ratio for its internal column layout

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
The system SHALL NOT display a tab bar.  The area previously used by the tab bar SHALL be allocated to the dual pane content area.

#### Scenario: No tab bar displayed
- **WHEN** the application is running
- **THEN** no tab bar is visible, and the full vertical space (minus header and status) is used for the two panes

### Requirement: Header and status bar adaptation
The Header SHALL display information for the active pane only.  The Status bar SHALL display information for the active pane only.

#### Scenario: Header shows active pane path
- **WHEN** the left pane is active and browsing /home/user/docs
- **THEN** the header displays the path /home/user/docs

#### Scenario: Switching active pane updates header
- **WHEN** the user switches from left pane (in /home/user/docs) to right pane (in /tmp)
- **THEN** the header updates to display /tmp
