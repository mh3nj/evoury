# Evoury Roadmap

This document outlines the development roadmap for Evoury, an offline-first creative asset manager.

---

## Vision

Evoury aims to be the definitive offline-first creative asset manager, combining the best features of Adobe Bridge, Eagle, PureRef, and modern file explorers into one cohesive, performant application.

---

## Development Phases

### Phase 1: Core Engine (Current)

**Goal**: Establish the foundational architecture and domain models.

#### Milestones

- [ ] **Domain Models**
  - Asset, Library, Folder, Collection
  - Workspace, Workstation, PreviewProfile
  - SearchProfile, ViewProfile, Theme
  - Command, Event, Capability

- [ ] **Event System**
  - Central Event Bus implementation
  - Publish/Subscribe architecture
  - Event types for all domain operations

- [ ] **Service Architecture**
  - Service interface definitions
  - Dependency injection patterns
  - Service lifecycle management

- [ ] **Registry System**
  - Runtime capability discovery
  - Plugin registration system
  - Dynamic feature loading

#### Deliverables

- Complete domain model crate (`atlas-core`)
- Event bus implementation (`atlas-events`)
- Service trait definitions
- Registry service

---

### Phase 2: Library Engine

**Goal**: Build a robust, scalable library management system.

#### Milestones

- [ ] **Scanner System**
  - Full scan mode
  - Incremental scan mode
  - Folder/file-specific scanning
  - Background scanning
  - Scan resumption and cancellation

- [ ] **Filesystem Watcher**
  - Real-time file system monitoring
  - Change detection (create, delete, rename, move)
  - Event-driven updates

- [ ] **Metadata Pipeline**
  - Extract metadata from files
  - Normalize metadata format
  - Validate metadata integrity
  - Cache processed metadata
  - Store in database

- [ ] **Preview System**
  - Async preview generation
  - Multi-resolution thumbnails
  - Memory and disk caching
  - Priority-based generation

- [ ] **Search Engine**
  - SQLite FTS5 integration
  - Full-text search
  - Field-based filtering
  - Saved searches
  - Search profiles

#### Deliverables

- Scanner service (`atlas-scanner`)
- Metadata pipeline (`atlas-metadata`)
- Search engine (`atlas-search`)
- Preview system (`atlas-preview`)
- Filesystem watcher integration

---

### Phase 3: User Experience & Workspace

**Goal**: Create a beautiful, customizable, and productive user interface.

#### Milestones

- [ ] **Workspace System**
  - Persistent workspace state
  - Multiple workspace support
  - Workspace switching

- [ ] **Workstations**
  - Pre-configured layouts
  - Tool configurations
  - Keyboard shortcuts
  - Theme presets

- [ ] **Layout Engine**
  - Dockable panels
  - Floating windows
  - Tabbed interfaces
  - Split views

- [ ] **Sidebar Navigation**
  - Library browser
  - Collection tree
  - Tag cloud
  - Favorites
  - Recent items
  - Project baskets

- [ ] **Inspector Panel**
  - Dynamic module loading
  - Asset-specific views
  - Metadata editing
  - Preview display

- [ ] **Command Palette**
  - Keyboard-first interface
  - Fuzzy search
  - Recent commands
  - Custom shortcuts

#### Deliverables

- Workspace manager (`atlas-workspace`)
- Workstation system (`atlas-workstation`)
- Layout engine (`atlas-layout`)
- Navigation system (`atlas-navigation`)
- Inspector components
- Command palette UI

---

### Phase 4: Advanced Features

**Goal**: Implement power-user features and advanced capabilities.

#### Milestones

- [ ] **Health Engine**
  - Filesystem health checks
  - Database integrity checks
  - Cache validation
  - Automatic repair

- [ ] **Duplicate Detection**
  - SHA256 hashing
  - Perceptual image hashing
  - Filename similarity
  - Duplicate manager UI

- [ ] **Batch Operations**
  - Bulk metadata editing
  - Batch renaming
  - Mass file operations
  - Undo/redo support

- [ ] **Plugin System**
  - Plugin API definition
  - Sandboxed execution
  - Plugin marketplace
  - Custom inspector modules

- [ ] **Advanced Search**
  - Query language parser
  - Boolean operators
  - Date ranges
  - Metadata comparisons

#### Deliverables

- Health service (`atlas-health`)
- Duplicate detector (`atlas-duplicate`)
- Batch processor (`atlas-batch`)
- Plugin system (`atlas-plugin`)
- Advanced search query engine

---

### Phase 5: Polish & Performance

**Goal**: Optimize performance, polish UI, and prepare for release.

#### Milestones

- [ ] **Performance Optimization**
  - Virtual scrolling
  - Lazy loading
  - Memory optimization
  - Startup time reduction

- [ ] **UI Polish**
  - Animations and transitions
  - Accessibility (a11y)
  - Responsive design
  - Dark/light themes

- [ ] **Testing**
  - Unit tests for all crates
  - Integration tests
  - E2E tests
  - Performance benchmarks

- [ ] **Documentation**
  - API documentation
  - User guide
  - Developer guide
  - Architecture documentation

- [ ] **Distribution**
  - Windows installer
  - macOS app bundle
  - Linux packages
  - Auto-updater

#### Deliverables

- Performance optimizations across all crates
- Complete test suite
- User documentation
- Developer documentation
- Distribution packages

---

## Future Considerations

### Potential Features (Post v1.0)

- **Cloud Sync**: Optional cloud backup/sync
- **Collaboration**: Multi-user libraries
- **AI Integration**: Smart tagging, search suggestions
- **Mobile Companion**: iOS/Android companion app
- **Web Interface**: Browser-based library access
- **API Access**: REST/GraphQL API for integrations
- **Scripting**: Lua/Python scripting support
- **Export/Import**: Library export and import tools

---

## Release Strategy

### Versioning

- **0.x**: Active development
- **1.0**: First stable release
- **1.x**: Feature additions
- **2.0**: Major architectural changes

### Release Cadence

- **Alpha**: Internal testing
- **Beta**: Public testing
- **Release Candidate**: Feature complete
- **Stable**: Production ready

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## Updates

This roadmap is a living document and will be updated as development progresses.

*Last updated: September 2026*
