## ADDED Requirements

### Requirement: Open action enters directory when target is a single directory
When the `open` action is invoked and the sole target is a directory, the system SHALL navigate into that directory (equivalent to the `enter` action) instead of opening it with an external program.

#### Scenario: Enter key on hovered directory
- **WHEN** user presses Enter with a single directory hovered and no files selected
- **THEN** the system SHALL navigate into that directory (same as pressing → or l)

#### Scenario: Enter key on hovered file
- **WHEN** user presses Enter with a file hovered
- **THEN** the system SHALL open the file with the associated program (unchanged behavior)

#### Scenario: Multiple items selected including directories
- **WHEN** user presses Enter with multiple items selected (files and/or directories)
- **THEN** the system SHALL open all selected items with associated programs (unchanged behavior)

#### Scenario: Open interactive mode on directory
- **WHEN** user invokes `open --interactive` on a directory
- **THEN** the system SHALL show the interactive picker (unchanged behavior, directory enter logic does NOT apply)
