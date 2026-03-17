## MODIFIED Requirements

### Requirement: Dual pane layout rendering
The system SHALL render two panes horizontally with a vertical separator between them when in dual-pane mode.  When in single-pane mode, the system SHALL render only the active pane at full width using the user's configured ratio (default `[1, 4, 3]`).  Each pane in dual-pane mode SHALL use ratio `[1, 2, 0]` by default (parent + current, no preview).  The two panes SHALL each occupy approximately 50% of the available width, minus the separator.

#### Scenario: Normal terminal width in dual-pane mode
- **WHEN** the terminal has sufficient width (>= 80 columns) and the system is in dual-pane mode
- **THEN** two panes are rendered side-by-side, each showing parent and current columns, separated by a vertical line

#### Scenario: Single-pane mode rendering
- **WHEN** the system is in single-pane mode
- **THEN** only the active pane is rendered at full width with parent, current, and preview columns

#### Scenario: Ratio configuration
- **WHEN** the user configures a custom ratio (e.g., `[1, 2, 1]`)
- **THEN** single-pane mode SHALL use the configured ratio; dual-pane mode SHALL use `[1, 2, 0]`

### Requirement: Tab bar removal
The system SHALL NOT display a tab bar.  The area previously used by the tab bar SHALL be allocated to the pane content area, whether in dual-pane or single-pane mode.

#### Scenario: No tab bar displayed
- **WHEN** the application is running in either mode
- **THEN** no tab bar is visible, and the full vertical space (minus header and status) is used for pane content
