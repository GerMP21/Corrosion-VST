# user-presets Specification

## Purpose
Defines how users save their own sounds as presets, where those presets live, and how they are listed and loaded alongside the factory bank.

## Requirements

### Requirement: Save the current sound as a user preset
The editor SHALL save the current parameters as a named preset file in the user's preset folder. A later load of that preset SHALL restore every parameter that a preset load sets.

#### Scenario: Save and reload
- **WHEN** the user saves the current sound as "My Grind", changes several parameters, then loads "My Grind"
- **THEN** every parameter returns to the value it had when saved, except UI Scale

#### Scenario: Name that is not a valid file name
- **WHEN** the user saves with a name containing path separators or characters invalid in file names
- **THEN** the preset is saved under a sanitized file name inside the user preset folder, keeps the typed name as its display name, and nothing is written outside that folder

### Requirement: Overwriting asks first
Saving under the name of an existing user preset SHALL ask for confirmation, and SHALL NOT change the file if the user declines.

#### Scenario: Declined overwrite
- **WHEN** the user saves under an existing user preset's name and declines the confirmation
- **THEN** the existing file is unchanged

### Requirement: User presets are listed after the factory bank
The editor's preset list SHALL show one submenu per factory folder, with the experimental folder nested by exciter, followed by a User submenu that shows the user's presets, including one saved during the current session. The User submenu SHALL mirror the subfolders of the user preset folder as nested submenus, and presets in the folder's root SHALL appear directly in it. The previous and next arrows SHALL step through the presets of the folder that holds the current preset.

#### Scenario: New preset appears
- **WHEN** the user saves a new preset
- **THEN** it appears in the User submenu, in the folder it was saved to, without reopening the editor

#### Scenario: Factory folders are submenus
- **WHEN** the preset list is opened
- **THEN** it shows the submenus Bass, Lead, Chords, Drones, Percussion and Experimental, then User, and Experimental opens one submenu per exciter

#### Scenario: User subfolders are submenus
- **WHEN** the user preset folder contains a subfolder `Basses` with a preset in it, and a preset in its root
- **THEN** the User submenu shows a `Basses` submenu holding that preset, and the root preset directly

#### Scenario: Arrows stay in the folder
- **WHEN** the current preset is the last one in the Bass folder and the next arrow is pressed
- **THEN** the first Bass preset loads

### Requirement: Save failures are reported
If a preset can't be saved, the editor SHALL show an error and SHALL NOT crash or block audio.

#### Scenario: Folder not writable
- **WHEN** the user preset folder can't be created or written
- **THEN** the editor shows an error message, and audio processing continues uninterrupted

### Requirement: Saving targets a user folder
Save SHALL write into the user preset folder or one of its subfolders, chosen in a folder selector next to the name. The selector SHALL offer only user folders, and no save SHALL ever write into the factory bank.

#### Scenario: Save into a subfolder
- **WHEN** the user picks the subfolder `Basses` and saves "Deep Sub"
- **THEN** the file is written in `Basses` inside the user preset folder, and "Deep Sub" appears under User › Basses

#### Scenario: The factory bank is never a target
- **WHEN** the folder selector is opened
- **THEN** it lists only the user preset folder and its subfolders

### Requirement: Open the user preset folder
The editor SHALL show an Open folder button next to the user presets. Pressing it SHALL create the user preset folder if it is missing and show it in the system file manager. A failure SHALL be reported in the editor, and SHALL NOT crash or block audio.

#### Scenario: The folder opens
- **WHEN** the user presses Open folder
- **THEN** the platform's file manager command is run with the user preset folder's path

#### Scenario: No preset folder available
- **WHEN** the platform gives no user preset folder (its variables are unset)
- **THEN** the button is disabled and its tooltip says why

### Requirement: Import preset files
The editor SHALL show an Import… button in the preset browser. It SHALL open the system file dialog for one or more `.corrosion-preset` files. Each chosen file that loads as a preset SHALL be copied into the user folder selected in the browser's folder selector. A name clash SHALL be resolved with a numbered file name, never by overwriting. The last imported preset SHALL load. Files that don't load SHALL be skipped and counted in a message. The factory bank SHALL never be a target.

#### Scenario: A preset downloaded from the web app
- **WHEN** the user imports a preset file downloaded from the web app
- **THEN** the file appears in the selected user folder, it is listed in the User submenu, and it loads

#### Scenario: Name clash
- **WHEN** the user imports "Deep Sub" into a folder that already holds "Deep Sub"
- **THEN** the new file is saved as "Deep Sub (2)" and the existing file is unchanged

#### Scenario: A file that isn't a preset
- **WHEN** the user imports two files and one of them doesn't parse as a preset
- **THEN** the valid one is imported and loaded, and the editor reports one file skipped

### Requirement: Export the current sound
The editor SHALL show an Export… button. It SHALL open the system save dialog, proposing the current preset's name with the `.corrosion-preset` extension, and SHALL write the current sound to the chosen path in the format Save writes. A file exported this way SHALL open in the web app and import back into the plugin with every parameter that a preset load sets.

#### Scenario: Round trip
- **WHEN** the user exports the current sound, changes several parameters, then imports the exported file
- **THEN** every parameter returns to the value it had when exported, except UI Scale

### Requirement: Dialog failures never block
If the platform has no file dialog, or the dialog fails, Import… and Export… SHALL report it in the editor with a pointer to Open folder. They SHALL NOT hang the editor, crash, or interrupt audio.

#### Scenario: No portal on Linux
- **WHEN** Import… is pressed on a Linux desktop where no file dialog answers
- **THEN** the editor shows a message naming Open folder as the alternative, and audio continues uninterrupted
