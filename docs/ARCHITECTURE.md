# Evoury Architecture

This document describes the high-level architecture of Evoury.

---

## Overview

Evoury is built with a modular, event-driven architecture that separates concerns and enables extensibility. The application consists of:

1. **Rust Backend**: Core logic, services, and data management
2. **React Frontend**: User interface and interaction
3. **Tauri Bridge**: Secure communication between backend and frontend

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      User Interface                         │
│                    (React + TypeScript)                      │
├─────────────────────────────────────────────────────────────┤
│                     Tauri IPC Bridge                         │
├─────────────────────────────────────────────────────────────┤
│                      Service Layer                          │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐          │
│  │ Scanner │ │ Preview │ │ Search  │ │ Health  │ ...       │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘          │
├─────────────────────────────────────────────────────────────┤
│                       Event Bus                             │
│              (Publish/Subscribe System)                      │
├─────────────────────────────────────────────────────────────┤
│                      Core Layer                             │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐          │
│  │ Domain  │ │ Registry│ │Scheduler│ │  Worker │ ...       │
│  │ Models  │ │         │ │         │ │  Pool   │          │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘          │
├─────────────────────────────────────────────────────────────┤
│                     Data Layer                              │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐                       │
│  │ SQLite  │ │  File   │ │  Cache  │                       │
│  │  (FTS5) │ │  System │ │  System │                       │
│  └─────────┘ └─────────┘ └─────────┘                       │
└─────────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. Domain Models (`atlas-core`)

Pure Rust data structures representing the application's domain:

```rust
// Example domain models
pub struct Asset {
    pub id: Uuid,
    pub path: PathBuf,
    pub state: AssetState,
    pub metadata: AssetMetadata,
}

pub struct Library {
    pub id: Uuid,
    pub name: String,
    pub root_path: PathBuf,
    pub settings: LibrarySettings,
}

pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub libraries: Vec<Uuid>,
    pub layout: Layout,
}
```

**Key Principles:**
- Pure data structures with no logic
- No external dependencies
- Serializable with serde
- UUID-based identity

### 2. Event Bus (`atlas-events`)

Decoupled communication system between services:

```rust
// Event types
pub enum Event {
    AssetAdded(Asset),
    AssetRemoved(Uuid),
    FolderScanned(PathBuf),
    PreviewUpdated(Uuid),
    SearchCompleted(Vec<Uuid>),
}

// Publish/Subscribe pattern
event_bus.publish(Event::AssetAdded(asset));
event_bus.subscribe::<AssetAdded>(|event| {
    // Handle event
});
```

**Key Principles:**
- Services never call each other directly
- All communication through events
- Supports async handlers
- Event history for debugging

### 3. Service Layer

Each service has a single responsibility:

```
┌─────────────────────────────────────────┐
│              Service Layer              │
├─────────────────────────────────────────┤
│ ScannerService    - File discovery      │
│ MetadataService   - Data extraction     │
│ PreviewService    - Thumbnail generation│
│ SearchService     - Query execution     │
│ HealthService     - Integrity checks    │
│ WorkspaceService  - Session state       │
│ CommandService    - User actions        │
│ RegistryService   - Feature discovery   │
└─────────────────────────────────────────┘
```

**Key Principles:**
- Single responsibility
- Public API only
- No direct service-to-service calls
- Async where appropriate

### 4. Registry System (`atlas-registry`)

Runtime discovery of capabilities:

```rust
// Capabilities registered at startup
registry.register(PreviewCapability);
registry.register(SearchCapability);
registry.register(HealthCapability);

// Query capabilities at runtime
if registry.supports::<PreviewCapability>() {
    // Show preview UI
}
```

### 5. Command System (`atlas-command`)

All user actions as commands:

```rust
pub trait Command {
    fn execute(&self) -> Result<()>;
    fn undo(&self) -> Result<()>;
    fn description(&self) -> &str;
}

// Commands accessible from:
// - Toolbar
// - Keyboard shortcuts
// - Command palette
// - Context menus
```

---

## Data Flow

### 1. Library Scanning

```
User Action (Scan Folder)
    ↓
Command: ScanFolderCommand
    ↓
ScannerService.scan()
    ↓
EventBus: FolderScanned
    ↓
MetadataService.extract()
    ↓
EventBus: MetadataExtracted
    ↓
SearchService.updateIndex()
    ↓
UI Updates
```

### 2. Asset Preview

```
User Selects Asset
    ↓
Command: SelectAssetCommand
    ↓
PreviewService.requestPreview()
    ↓
Check Memory Cache
    ↓ (miss)
Check Disk Cache
    ↓ (miss)
Generate Thumbnail
    ↓
Store in Cache
    ↓
EventBus: PreviewReady
    ↓
UI Displays Preview
```

### 3. Search

```
User Types Query
    ↓
Command: SearchCommand
    ↓
SearchService.search()
    ↓
Query SQLite FTS5
    ↓
Return Results
    ↓
EventBus: SearchCompleted
    ↓
UI Displays Results
```

---

## Crate Organization

### Core Crates

| Crate | Purpose |
|-------|---------|
| `atlas-core` | Domain models and types |
| `atlas-events` | Event bus system |
| `atlas-registry` | Capability discovery |
| `atlas-command` | Command system |
| `atlas-scheduler` | Task scheduling |

### Service Crates

| Crate | Purpose |
|-------|---------|
| `atlas-scanner` | File system scanning |
| `atlas-metadata` | Metadata extraction |
| `atlas-preview` | Preview generation |
| `atlas-search` | Search engine |
| `atlas-health` | Health checks |
| `atlas-duplicate` | Duplicate detection |

### Workspace Crates

| Crate | Purpose |
|-------|---------|
| `atlas-workspace` | Workspace management |
| `atlas-workstation` | Workstation presets |
| `atlas-layout` | Layout engine |
| `atlas-navigation` | Sidebar navigation |
| `atlas-selection` | Selection management |

### Infrastructure Crates

| Crate | Purpose |
|-------|---------|
| `atlas-db` | SQLite database |
| `atlas-cache` | Caching system |
| `atlas-worker` | Worker pool |
| `atlas-notification` | Notifications |

---

## Frontend Architecture

### Component Structure

```
src/
├── components/
│   ├── common/          # Shared components
│   ├── gallery/         # Gallery view
│   ├── inspector/       # Inspector panel
│   ├── sidebar/         # Navigation sidebar
│   ├── toolbar/         # Top toolbar
│   ├── search/          # Search components
│   └── modals/          # Modal dialogs
├── hooks/               # Custom React hooks
├── stores/              # Zustand state stores
├── pages/               # Page components
├── styles/              # Global styles
├── types/               # TypeScript types
└── i18n/                # Internationalization
```

### State Management

Using Zustand for state management:

```typescript
// Example store
interface AppStore {
  // State
  assets: Asset[];
  selectedAsset: Asset | null;
  searchQuery: string;
  
  // Actions
  selectAsset: (asset: Asset) => void;
  setSearchQuery: (query: string) => void;
}
```

### Tauri IPC

Communication with Rust backend:

```typescript
// Calling Rust commands
const assets = await invoke<Asset[]>('get_assets');

// Listening to events
listen('asset-updated', (event) => {
  updateAsset(event.payload);
});
```

---

## Performance Considerations

### Virtual Scrolling

Large libraries use virtual scrolling:
- Only render visible items
- Dynamic item heights
- Smooth scrolling

### Lazy Loading

- Thumbnails load on demand
- Metadata fetched as needed
- Preview generation prioritized

### Caching Strategy

```
Memory Cache (LRU)
    ↓ (miss)
Disk Cache
    ↓ (miss)
Generate & Cache
```

### Background Workers

- Scanner worker for file operations
- Preview worker for thumbnail generation
- Index worker for search updates
- Health worker for integrity checks

---

## Security Model

### Tauri Security

- CSP (Content Security Policy) configured
- IPC validation
- File system access control

### Data Isolation

- All data stored locally
- No network requests without explicit permission
- User controls all file access

---

## Future Considerations

### Plugin System

- Sandboxed plugin execution
- API surface definition
- Plugin marketplace

### Cloud Sync (Optional)

- Encrypted sync
- Conflict resolution
- Offline-first with optional sync

### Multi-User Support

- Shared libraries
- Permission system
- Collaboration features

---

## References

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [React Documentation](https://react.dev/)
- [Event-Driven Architecture](https://martinfowler.com/articles/201701-event-driven.html)

---

*Last updated: September 2026*
