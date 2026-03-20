## ADDED Requirements

### Requirement: Sync directory to other pane
The system SHALL provide a `pane_sync_dir` action that navigates the other pane's active tab to the same directory as the current pane's active tab.  The active pane SHALL remain unchanged after the operation.

#### Scenario: Sync left pane directory to right pane
- **WHEN** the left pane is active in /home/user/docs and the right pane is in /tmp, and the user triggers `pane_sync_dir`
- **THEN** the right pane navigates to /home/user/docs, and the left pane remains active

#### Scenario: Sync right pane directory to left pane
- **WHEN** the right pane is active in /var/log and the left pane is in /home, and the user triggers `pane_sync_dir`
- **THEN** the left pane navigates to /var/log, and the right pane remains active

#### Scenario: Both panes already in same directory
- **WHEN** both panes are already in /home/user, and the user triggers `pane_sync_dir`
- **THEN** the operation is a no-op, and the active pane remains unchanged

#### Scenario: Sync with multiple tabs in other pane
- **WHEN** the left pane is active in /home/user, and the right pane has 3 tabs with tab 2 active in /tmp
- **THEN** the right pane's tab 2 navigates to /home/user, other tabs are unaffected
