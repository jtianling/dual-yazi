## MODIFIED Requirements

### Requirement: Dual pane initialization
The system SHALL initialize with exactly two panes on startup.  Each pane SHALL contain one tab.  The left pane (index 0) SHALL be the active pane by default.  Both panes SHALL start in the user's current working directory.

#### Scenario: Application startup
- **WHEN** the application starts
- **THEN** two panes are displayed side-by-side, each with one tab, left pane is active, both showing the current working directory

### Requirement: Dual pane layout rendering
The system SHALL render two panes horizontally with a vertical separator between them when in dual-pane mode. Each pane SHALL be a self-contained vertical layout consisting of: a Header row, an optional Tab bar (shown when the pane has more than one tab), the Tab content area, and a Status bar row. When in single-pane mode, the system SHALL render only the active pane at full width using the user's configured ratio (default `[1, 4, 3]`) with root-level Header, optional Tab bar, and Status. Each pane in dual-pane mode SHALL use ratio `[1, 2, 0]` by default (parent + current, no preview). When preview_pane mode is enabled, each pane SHALL use ratio `[0, 1, 1]` (current + preview, no parent). The two panes SHALL each occupy approximately 50% of the available width, minus the separator. The root layout in dual-pane mode SHALL allocate the full screen height to the DualPane component (no root-level Header or Status rows).

#### Scenario: Normal terminal width in dual-pane mode
- **WHEN** the terminal has sufficient width (>= 80 columns) and the system is in dual-pane mode with preview_pane disabled
- **THEN** two panes are rendered side-by-side, each with its own header, parent and current columns, and status bar, separated by a vertical line

#### Scenario: Dual-pane with multiple tabs in one pane
- **WHEN** the system is in dual-pane mode and the left pane has 3 tabs while the right pane has 1 tab
- **THEN** the left pane renders header, tab bar, content, and status; the right pane renders header, content, and status (no tab bar)

#### Scenario: Single-pane mode rendering
- **WHEN** the system is in single-pane mode
- **THEN** only the active pane is rendered at full width with root-level header, optional tab bar, parent, current, and preview columns, and root-level status bar

### Requirement: Tab bar removal
**This requirement is removed.**

## REMOVED Requirements

### Requirement: Tab bar removal
**Reason**: Tab bar is now conditionally displayed per pane when the pane has more than one tab.
**Migration**: The tab bar will appear automatically when a pane has multiple tabs.  No user action needed.
