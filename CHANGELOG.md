# Changelog

## [2.0.1] - 2025-03-07

### Fixed

- bug which caused empty output if no tera tag was present.

## [2.0.0] - 2025-03-07

Introduced template enging tag: `<tera src="./relative-path/to/config.toml" name="optional"></tera>`

### Added

- #Feature. now it is possible to proccess template engine variables via **tera** package.

### Changed

- #Lib. markdown processor is now **markdown** instead of **comrk**.
