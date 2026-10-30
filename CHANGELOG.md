# Changelog

All notable changes to Evoury will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Several library folders.** Add as many as you like (**Libraries** in the top bar): rename, rescan or remove each one. Everything is shown together. Removing a library only forgets it, files are never touched. A folder that is offline (unplugged drive, share down) is reported and keeps its assets instead of emptying the library, and overlapping folders (one inside another) are refused. Asset ids include the library, so two libraries holding `Fonts/Logo` don't collide; the first library's ids are unchanged
- **Instant start-up.** The library is stored in a SQLite index (`atlas-db`, migration 2). On launch it appears straight from the index with no scan, then a quick background scan catches what changed. Collections and categories you assigned are kept across rescans and restarts
- **Incremental scanning that actually skips work.** The scanner remembers each folder's modification time and re-lists only folders that changed (a folder changed in the last 2 seconds is never trusted, so nothing is missed). _Rescan_ in the Libraries dialog (and `verify` mode) still checks every file. The watcher uses the same rule, so a change in a big library costs a rescan of one folder
- **Duplicates** (new top-bar button). Finds (1) byte-identical archives, hashed in stages (size, then three 1 MiB samples, then a full BLAKE2 only for survivors) and cached, (2) packages that mostly contain the same files, including the same pack saved twice or in two formats, and (3) files that sit inside several packages, such as a font bundled into twelve mockups. It shows where the copies are and never deletes anything. Items 2 and 3 use each package's contents: Kaelovun's manifest (now with CRC32 per file) gives them instantly, otherwise 7-Zip reads each archive once and Evoury remembers
- **Content-Security-Policy.** `csp` was `null`. It now allows only the app's own scripts, bundled styles and fonts, and previews through the asset protocol. No network access is allowed (the app doesn't use any)
- **Archive contents viewer.** The viewer's new folder button (and the `zip-tree` viewer) shows what is inside an archive without extracting it: a folder tree with sizes, file counts, categories and a filter box. It reads Kaelovun's `<archive>.kaelovun.json` sidecar when there is one (instant, works for RAR) and otherwise lists the archive with 7-Zip (`7z` on PATH, `C:\Program Files\7-Zip`, or the `EVOURY_7Z` variable). Nothing is ever extracted, and the backend takes an asset id, not a path
- **7z and split archives.** `Name.part1.rar` / `Name.7z.001` sets are one asset (only the first volume is indexed), and `.tar.gz`/`.tgz`/`.tar.bz2`/`.tar.xz` names lose their full extension
- **Thumbnails.** `Name.thumb.avif` is paired with its archive and used in the grid; the full preview is used everywhere else
- `get_archive_contents` command, `atlas_core::archive` naming rules, `Library::sync`

### Fixed

- **Tags, ratings and favourites were orphaned on every restart.** Asset ids were random on every scan, so everything saved against an id pointed at nothing after the next launch or rescan. Ids are now derived from the package's folder and name, so they are stable, survive moving the library folder, and do not change when `Pack.rar` becomes `Pack.7z`
- **Rescanning duplicated the whole library** and kept assets whose files had been deleted. A scan now updates assets in place (keeping their collection and category) and removes the ones that are gone
- **Two packages with the same name in different folders became one.** Archives and previews were matched on the bare file name across the whole library, so the second `Logo.rar` vanished and a preview could be paired with the wrong archive. Matching is now per folder
- **New archives never appeared until a manual rescan.** The watcher reported changes but nothing updated the library. The app now rescans once things settle (1.5 s after the last change)
- **Unfinished Kaelovun files were indexed.** `Name.kaelovun-part.7z` is a half-written archive; it, its temporary preview and macOS `._` files are ignored by the scanner and the watcher
- When a package exists in two formats, one is chosen deterministically (newest file, then 7z > RAR > ZIP) instead of whichever the folder walk met first
- **Quick Preview, Compare and the viewer showed broken images.** They used the raw file path as an image source, which the webview blocks; only grid cards and the presentation went through the asset protocol. One helper now does this everywhere
- Scanning no longer freezes the window (the command is async), and recycle-bin / `.git` / `__MACOSX` folders are skipped
- The sidebar's folder categories and collections keep their ids across rescans instead of being regenerated

### Security

- **Notes could run script.** A note's text was turned into HTML without escaping, so a note containing `<img src=x onerror=...>` (typed, pasted, or imported) executed inside the app, which has access to every backend command. Notes are now escaped before the markdown is applied; the new CSP also blocks inline handlers
- The webview is granted access to the library folder when it is scanned. `assetScope` in `tauri.conf.json` is still `**` (the whole disk); it can now be narrowed to `[]` in `src-tauri/tauri.conf.json` without breaking previews, because each scanned library folder is allowed at runtime

## [0.1.2] - 2026-10-11

### Fixed

- Settings changes now reach the whole app on startup, so a saved view mode, density, layout or card size is what you see on the next launch
- The library watcher and scan progress no longer re-register their event listener on every progress tick
- Autostart on Linux quotes the executable path, so it works when the app is installed in a folder with a space in its name
- The old unused toolbar component was removed; the library scan button only exists in the top bar

### Changed

- Every CI and release job now builds with a pinned Rust toolchain, so the checks and the installers use the same compiler on Windows, macOS and Linux
- Rust, TypeScript and formatting checks run clean with no warnings

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

[Unreleased]: https://github.com/evoury/evoury/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/evoury/evoury/releases/tag/v0.1.2
[0.1.1]: https://github.com/evoury/evoury/releases/tag/v0.1.1
[0.1.0]: https://github.com/evoury/evoury/releases/tag/v0.1.0
