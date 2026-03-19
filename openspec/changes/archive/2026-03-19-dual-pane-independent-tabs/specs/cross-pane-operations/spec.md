## MODIFIED Requirements

### Requirement: Copy to other pane
The `copy_to` action SHALL copy selected or hovered files from the active pane's active tab to the other pane's active tab's current working directory.

#### Scenario: Copy files to other pane's active tab
- **WHEN** the user selects files in the left pane's active tab and presses F5
- **THEN** the files are copied to the right pane's active tab's current working directory

#### Scenario: Copy when other pane has multiple tabs
- **WHEN** the right pane has 3 tabs with tab 2 active, and the user copies from the left pane
- **THEN** the files are copied to the right pane's tab 2's current working directory (not tab 1 or tab 3)

### Requirement: Move to other pane
The `move_to` action SHALL move selected or hovered files from the active pane's active tab to the other pane's active tab's current working directory.

#### Scenario: Move files to other pane's active tab
- **WHEN** the user selects files in the left pane's active tab and presses F6
- **THEN** the files are moved to the right pane's active tab's current working directory

#### Scenario: Move when other pane has multiple tabs
- **WHEN** the left pane has 2 tabs with tab 1 active, and the right pane has 3 tabs with tab 3 active, and the user moves from the left pane
- **THEN** the files are moved to the right pane's tab 3's current working directory
