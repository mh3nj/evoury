# TODO

This document tracks development tasks and priorities.

---

## Priority Levels

- **P0**: Critical - Must be done immediately
- **P1**: High - Should be done soon
- **P2**: Medium - Can be done in regular development
- **P3**: Low - Nice to have, can wait

---

## Phase 1: Core Engine

### Domain Models (atlas-core)

- [ ] P0: Implement Asset model
- [ ] P0: Implement Library model
- [ ] P0: Implement Folder model
- [ ] P0: Implement Collection model
- [ ] P1: Implement Workspace model
- [ ] P1: Implement Workstation model
- [ ] P1: Implement PreviewProfile model
- [ ] P1: Implement SearchProfile model
- [ ] P2: Implement ViewProfile model
- [ ] P2: Implement Theme model
- [ ] P2: Implement Command model
- [ ] P2: Implement Event model
- [ ] P3: Implement Capability model

### Event System (atlas-events)

- [ ] P0: Implement Event Bus
- [ ] P0: Implement publish/subscribe pattern
- [ ] P1: Add async event handlers
- [ ] P1: Add event history logging
- [ ] P2: Add event filtering
- [ ] P3: Add event replay

### Service Architecture

- [ ] P0: Define service traits
- [ ] P0: Implement service lifecycle
- [ ] P1: Add dependency injection
- [ ] P2: Add service health monitoring
- [ ] P3: Add service metrics

### Registry System (atlas-registry)

- [ ] P0: Implement capability registration
- [ ] P1: Add runtime discovery
- [ ] P2: Add plugin registration
- [ ] P3: Add dynamic loading

---

## Phase 2: Library Engine

### Scanner Service (atlas-scanner)

- [ ] P0: Implement full scan mode
- [ ] P0: Implement incremental scan
- [ ] P1: Implement folder-specific scan
- [ ] P1: Implement file-specific scan
- [ ] P2: Implement background scan
- [ ] P2: Add scan resumption
- [ ] P2: Add scan cancellation
- [ ] P3: Implement verification scan
- [ ] P3: Implement repair scan

### Filesystem Watcher

- [ ] P0: Implement file creation detection
- [ ] P0: Implement file deletion detection
- [ ] P1: Implement file rename detection
- [ ] P1: Implement file move detection
- [ ] P2: Implement file modification detection
- [ ] P3: Implement permission change detection

### Metadata Pipeline (atlas-metadata)

- [ ] P0: Implement metadata extraction
- [ ] P0: Implement metadata normalization
- [ ] P1: Implement metadata validation
- [ ] P1: Implement metadata caching
- [ ] P2: Implement metadata storage
- [ ] P3: Add plugin metadata providers

### Preview System (atlas-preview)

- [ ] P0: Implement thumbnail generation
- [ ] P0: Implement memory cache
- [ ] P1: Implement disk cache
- [ ] P1: Add multi-resolution support
- [ ] P2: Add priority-based generation
- [ ] P3: Add generation cancellation

### Search Engine (atlas-search)

- [ ] P0: Implement SQLite FTS5
- [ ] P0: Implement full-text search
- [ ] P1: Implement field-based filtering
- [ ] P1: Implement saved searches
- [ ] P2: Implement search profiles
- [ ] P2: Add autocomplete
- [ ] P3: Add search history
- [ ] P3: Implement advanced query language

### Duplicate Detection (atlas-duplicate)

- [ ] P1: Implement SHA256 hashing
- [ ] P1: Implement filename similarity
- [ ] P2: Implement perceptual image hashing
- [ ] P2: Implement metadata comparison
- [ ] P3: Add duplicate manager UI

---

## Phase 3: User Experience

### Workspace System (atlas-workspace)

- [ ] P0: Implement workspace persistence
- [ ] P0: Implement workspace restoration
- [ ] P1: Implement multiple workspaces
- [ ] P1: Implement workspace switching
- [ ] P2: Add workspace templates

### Workstation System (atlas-workstation)

- [ ] P1: Implement workstation presets
- [ ] P1: Implement layout presets
- [ ] P2: Implement tool presets
- [ ] P2: Implement shortcut presets
- [ ] P3: Implement theme presets

### Layout Engine (atlas-layout)

- [ ] P0: Implement panel docking
- [ ] P1: Implement floating panels
- [ ] P1: Implement tabbed interfaces
- [ ] P2: Implement split views
- [ ] P3: Implement drag-and-drop

### Sidebar Navigation (atlas-navigation)

- [ ] P0: Implement library browser
- [ ] P0: Implement collection tree
- [ ] P1: Implement tag cloud
- [ ] P1: Implement favorites
- [ ] P2: Implement recent items
- [ ] P3: Implement project baskets

### Inspector Panel

- [ ] P1: Implement dynamic module loading
- [ ] P1: Implement asset-specific views
- [ ] P2: Implement metadata editing
- [ ] P2: Implement preview display
- [ ] P3: Add plugin inspector modules

### Command Palette

- [ ] P1: Implement command search
- [ ] P1: Implement fuzzy search
- [ ] P2: Implement recent commands
- [ ] P3: Implement custom shortcuts

---

## Phase 4: Advanced Features

### Health Engine (atlas-health)

- [ ] P1: Implement filesystem checks
- [ ] P1: Implement database checks
- [ ] P2: Implement cache validation
- [ ] P2: Implement automatic repair
- [ ] P3: Add health reports

### Batch Operations (atlas-batch)

- [ ] P2: Implement bulk metadata editing
- [ ] P2: Implement batch renaming
- [ ] P3: Implement mass file operations
- [ ] P3: Add undo/redo support

### Plugin System (atlas-plugin)

- [ ] P3: Define plugin API
- [ ] P3: Implement sandboxed execution
- [ ] P3: Add plugin marketplace

---

## Phase 5: Polish & Performance

### Performance Optimization

- [ ] P1: Implement virtual scrolling
- [ ] P1: Implement lazy loading
- [ ] P2: Optimize memory usage
- [ ] P2: Reduce startup time
- [ ] P3: Add performance benchmarks

### UI Polish

- [ ] P2: Add animations
- [ ] P2: Add accessibility (a11y)
- [ ] P2: Add responsive design
- [ ] P3: Add dark/light themes

### Testing

- [ ] P1: Add unit tests
- [ ] P1: Add integration tests
- [ ] P2: Add E2E tests
- [ ] P3: Add performance tests

### Documentation

- [ ] P1: Add API documentation
- [ ] P1: Add user guide
- [ ] P2: Add developer guide
- [ ] P3: Add architecture diagrams

### Distribution

- [ ] P1: Create Windows installer
- [ ] P2: Create macOS app bundle
- [ ] P2: Create Linux packages
- [ ] P3: Add auto-updater

---

## Completed

- [x] Project structure setup
- [x] Initial documentation
- [x] GitHub templates
- [x] CI/CD pipeline

---

*Last updated: September 2026*
