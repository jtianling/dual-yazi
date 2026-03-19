# Capability: Per Pane Tabs

## Purpose

Defines the independent per-pane tab list architecture, where each pane maintains its own set of tabs and all tab operations are scoped to the currently active pane.

## Requirements

### Requirement: Pane data structure contains independent tab list
Each pane SHALL maintain its own independent list of tabs and its own cursor index.  The system SHALL have exactly two panes.  Each pane SHALL be initialized with one tab.

#### Scenario: Application startup
- **WHEN** the application starts
- **THEN** two panes exist, each containing exactly one tab, with the left pane (index 0) active

#### Scenario: Pane tab independence
- **WHEN** the user creates a new tab in the left pane
- **THEN** only the left pane's tab list grows; the right pane's tab list is unchanged

### Requirement: Tab create operates on active pane
The `tab_create` action SHALL create a new tab within the currently active pane's tab list.  The new tab SHALL open in the specified directory or the current working directory of the active tab.

#### Scenario: Create tab in left pane
- **WHEN** the left pane is active and the user invokes tab_create
- **THEN** a new tab is added to the left pane's tab list and becomes the active tab within that pane

#### Scenario: Create tab in right pane
- **WHEN** the right pane is active and the user invokes tab_create
- **THEN** a new tab is added to the right pane's tab list and becomes the active tab within that pane

### Requirement: Tab close operates on active pane
The `tab_close` action SHALL close a tab within the currently active pane's tab list.  If only one tab remains in the pane, the close action SHALL be a no-op.

#### Scenario: Close tab with multiple tabs
- **WHEN** the active pane has 3 tabs and the user closes the second tab
- **THEN** the second tab is removed, the pane has 2 tabs, and the cursor adjusts appropriately

#### Scenario: Close last remaining tab
- **WHEN** the active pane has exactly 1 tab and the user invokes tab_close
- **THEN** nothing happens (no-op)

### Requirement: Tab switch operates on active pane
The `tab_switch` action SHALL switch between tabs within the currently active pane.  Switching tabs SHALL NOT change the active pane.

#### Scenario: Switch to next tab in pane
- **WHEN** the active pane has 3 tabs with tab 1 active and the user invokes tab_switch with step=1
- **THEN** tab 2 becomes active within the same pane

#### Scenario: Tab switch wraps around
- **WHEN** the active pane has 3 tabs with tab 3 active and the user invokes tab_switch with step=1
- **THEN** tab 1 becomes active within the same pane

### Requirement: Tab swap operates on active pane
The `tab_swap` action SHALL swap tab positions within the currently active pane's tab list.

#### Scenario: Swap tab forward
- **WHEN** the active pane has 3 tabs with tab 1 active and the user invokes tab_swap with step=1
- **THEN** tab 1 and tab 2 exchange positions within the pane, cursor follows the swapped tab

### Requirement: Tab rename operates on active pane
The `tab_rename` action SHALL rename the active tab within the currently active pane.

#### Scenario: Rename tab
- **WHEN** the active pane has a tab named "docs" and the user renames it to "references"
- **THEN** the tab name changes to "references" within the pane

### Requirement: Active pane context for tab operations
All tab operations (create, close, switch, swap, rename) SHALL use the currently active pane as their scope.  The `cx.tab()`, `cx.tabs()`, `cx.cwd()` and similar context methods SHALL resolve to the active pane's active tab.

#### Scenario: cx.tab returns active pane's active tab
- **WHEN** the left pane is active with tab 2 selected (out of 3)
- **THEN** `cx.tab()` returns the left pane's tab 2

#### Scenario: cx.cwd returns active pane's active tab's cwd
- **WHEN** the right pane is active with its active tab browsing /tmp
- **THEN** `cx.cwd()` returns /tmp

### Requirement: Tab bar display per pane
When a pane has more than one tab, the system SHALL display a tab bar within that pane showing all tab names and indicating the active tab.  When a pane has exactly one tab, no tab bar SHALL be displayed.

#### Scenario: Multiple tabs show tab bar
- **WHEN** the left pane has 3 tabs
- **THEN** the left pane displays a tab bar between its header and content area

#### Scenario: Single tab hides tab bar
- **WHEN** the right pane has exactly 1 tab
- **THEN** the right pane does NOT display a tab bar
