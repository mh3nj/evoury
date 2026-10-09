# Changelog

All notable changes to Evoury will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-10-10

### Fixed
- Settings changes now persist and apply to the app (theme, density, layout and card size were not saved before)
- Preferences are written as a complete payload so a single change no longer resets the rest
- Duplicate scan button removed; the library scan lives in the top bar only
- Folder reveal labels now match the operating system (Explorer, Finder or file manager)

### Changed
- Gallery density and layout are driven by the saved preferences
- Screenshot and license text cleaned up in all translated readmes

## [Unreleased]

### Added
- Initial project structure
- Core domain models
- Event bus system
- Service architecture
- Registry system
- Command system
- React frontend with TypeScript
- Tauri integration
- Documentation

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - 2026-XX-XX

### Added
- Core engine implementation
- Domain models for Asset, Library, Workspace
- Event bus with publish/subscribe pattern
- Service layer architecture
- Registry for capability discovery
- Command system for user actions
- Basic scanner service
- Metadata extraction
- Preview generation
- Search engine with SQLite FTS5
- Health check system
- Workspace persistence
- Layout engine
- React frontend
- Tauri desktop integration
- Documentation and guides

[Unreleased]: https://github.com/evoury/evoury/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/evoury/evoury/releases/tag/v0.1.1
[0.1.0]: https://github.com/evoury/evoury/releases/tag/v0.1.0
