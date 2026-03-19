## MODIFIED Requirements

### Requirement: Tab action disabled in dual-pane mode
The system SHALL support `tab_create`, `tab_close`, `tab_switch`, `tab_swap` actions in dual-pane mode, operating on the active pane's tab list.  The `t` key SHALL be bound to `tab_create --current` as a single key press (not a chord).  The `tab_rename` action SHALL have no default keybinding but SHALL remain available for user customization.

#### Scenario: Single key t creates tab
- **WHEN** the user presses `t`
- **THEN** a new tab is immediately created in the active pane's current working directory

#### Scenario: r key renames file
- **WHEN** the user presses `r`
- **THEN** the rename action is invoked for the selected file(s), not tab_rename

#### Scenario: tab_rename has no default binding
- **WHEN** the user inspects the default keymap
- **THEN** there is no keybinding for `tab_rename --interactive`
