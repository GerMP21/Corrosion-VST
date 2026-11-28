# Spec Delta

## ADDED Requirements

### Requirement: Oversized preset files are refused unread
A preset file larger than 4 MB SHALL be refused before its contents are read, both when it is imported and when the user preset folder is scanned. Import SHALL count it as skipped in its message; the folder scan SHALL leave it out of the browser as it does any file that doesn't load.

#### Scenario: Importing a huge file by mistake
- **WHEN** the user imports a valid preset together with a 5 MB file named `big.corrosion-preset`
- **THEN** the valid preset is imported and loaded, the editor reports one file skipped, and the large file's contents are never read into memory

#### Scenario: The largest legitimate file still loads
- **WHEN** a preset holding a 128-pad kit (about 0.5 MB) is imported
- **THEN** it imports and loads normally
