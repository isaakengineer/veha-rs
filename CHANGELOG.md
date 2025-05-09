# Changelog

## [4.1.1] - 2025-05-09

### Added

- #Fähigkeit. vernetzte Bearbeitung von Motoren nach Backend: vernetzte Werte durch SQLite-Datensätze werden von anderen Motoren weiterbearbeitet

### Fixed

- #Fehlerbehebung. Pfad der SQLite-Datei: relative Pfad ist jetzt erkennbar.

## [4.1.0] - 2025-04-11

### Added

- #Fähigkeit. `<backend>`Tag mit entsprechenden Gestalltung bezüglich der Architektur des Bindervorgehenweise

### Changed

- #Hausputz. verbesesrte Modulisierung des Schriftes/Kodes

## [3.0.0] - 2025-03-23

### Added

- #Feature. if `multilingual` attribute exist on veha-introduced tags, they will be processed via `lang` input.
- #Feature. possibility to add **map** (via a CSV file) to covert a series of structures into final web pages, processed under the _site_ subcommand.

### Changed

- the default behavior is now part of the _page_ subcommand.

### Fixed

- Fixed an issue where the Toml processing engine couldn't process more than a single _toml_ tag.

## [2.0.1] - 2025-03-07

### Fixed

- bug which caused empty output if no tera tag was present.

## [2.0.0] - 2025-03-07

Introduced template enging tag: `<tera src="./relative-path/to/config.toml" name="optional"></tera>`

### Added

- #Feature. now it is possible to proccess template engine variables via **tera** package.

### Changed

- #Lib. markdown processor is now **markdown** instead of **comrk**.
