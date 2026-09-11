use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use tauri::Manager;
use atlas_basket::BasketManager;
use atlas_events::EventBus;
use atlas_library::Library;
use atlas_metadata::{MetadataStore, TagManager, CustomFieldManager, TemplateManager, AutoClassifier};
use atlas_preferences::PreferenceStore;
use atlas_scanner::FilesystemWatcher;
use atlas_registry::{Registry, Provider, ProviderKind};
use atlas_scheduler::Scheduler;
use atlas_power::SleepManager;
use atlas_workspace::{WorkspaceManager, SessionRecovery};
use atlas_command::{CommandRegistry, CommandExecutor};
use atlas_cache::CacheManager;
use atlas_worker::WorkerPool;
use atlas_duplicate::DuplicateDetector;
use atlas_collection::CollectionManager;
use atlas_links::{VersionDetector, RelationshipManager, DependencyGraph, VersionManager};
use atlas_history::{ActivityTracker, VersionHistory};
use atlas_project::ProjectManager;
use atlas_search::SavedSearchManager;
use atlas_undo::UndoStack;
use atlas_batch::{BatchRenamer, BatchMetadataEditor};
use atlas_pipeline::PipelineManager;
use atlas_macro::{MacroRecorder, MacroPlayback};
use atlas_notification::NotificationManager;
use atlas_health::MaintenanceEngine;
use atlas_db::runner::MigrationRunner;
use atlas_plugin::PluginRegistry;
use atlas_diagnostics::DiagnosticCollector;
use atlas_backup::BackupManager;
use atlas_haystack::LargeLibraryConfig;
use atlas_layout::LayoutRegistry;
use atlas_workstation::WorkstationRegistry;
use atlas_profiles::ProfileManager;
use atlas_selection::SelectionEngine;
use atlas_navigation::{NavigationHistory, TabManager, BreadcrumbTrail};
use atlas_keyboard::ShortcutRegistry;
use atlas_animation::AnimationManager;
use atlas_preview::PreviewManager;
use atlas_viewer::ViewerRegistry;
use atlas_compare::CompareEngine;
use atlas_gpu::GpuTracker;
use atlas_color::ColorManager;
use crate::persistence::Persistence;

pub struct AppState {
    pub library: Arc<Library>,
    pub metadata_store: Arc<Mutex<MetadataStore>>,
    pub basket_manager: Arc<Mutex<BasketManager>>,
    pub event_bus: Arc<EventBus>,
    pub watcher: Arc<Mutex<FilesystemWatcher>>,
    pub persistence: Arc<Mutex<Persistence>>,
    pub preferences: Arc<Mutex<PreferenceStore>>,
    pub registry: Arc<Mutex<Registry>>,
    pub scheduler: Arc<Scheduler>,
    pub sleep_manager: Arc<Mutex<SleepManager>>,
    pub workspace_manager: Arc<Mutex<WorkspaceManager>>,
    pub session_recovery: Arc<Mutex<SessionRecovery>>,
    pub command_registry: Arc<Mutex<CommandRegistry>>,
    pub command_executor: Arc<CommandExecutor>,
    pub cache_manager: Arc<Mutex<CacheManager>>,
    pub worker_pool: Arc<Mutex<WorkerPool>>,
    pub duplicate_detector: Arc<DuplicateDetector>,
    pub collection_manager: Arc<Mutex<CollectionManager>>,
    pub version_detector: Arc<VersionDetector>,
    pub large_library_config: LargeLibraryConfig,
    pub layout_registry: Arc<Mutex<LayoutRegistry>>,
    pub workstation_registry: Arc<Mutex<WorkstationRegistry>>,
    pub profile_manager: Arc<Mutex<ProfileManager>>,
    pub selection_engine: Arc<Mutex<SelectionEngine>>,
    pub nav_history: Arc<Mutex<NavigationHistory>>,
    pub tab_manager: Arc<Mutex<TabManager>>,
    pub breadcrumb_trail: Arc<Mutex<BreadcrumbTrail>>,
    pub shortcut_registry: Arc<Mutex<ShortcutRegistry>>,
    pub animation_manager: Arc<Mutex<AnimationManager>>,
    pub preview_manager: Arc<PreviewManager>,
    pub viewer_registry: Arc<ViewerRegistry>,
    pub compare_engine: Arc<Mutex<CompareEngine>>,
    pub gpu_tracker: Arc<Mutex<GpuTracker>>,
    pub color_manager: Arc<Mutex<ColorManager>>,
    pub tag_manager: Arc<Mutex<TagManager>>,
    pub custom_field_manager: Arc<Mutex<CustomFieldManager>>,
    pub template_manager: Arc<Mutex<TemplateManager>>,
    pub auto_classifier: Arc<Mutex<AutoClassifier>>,
    pub relationship_manager: Arc<Mutex<RelationshipManager>>,
    pub dependency_graph: Arc<Mutex<DependencyGraph>>,
    pub links_version_manager: Arc<Mutex<VersionManager>>,
    pub activity_tracker: Arc<Mutex<ActivityTracker>>,
    pub version_history: Arc<Mutex<VersionHistory>>,
    pub project_manager: Arc<Mutex<ProjectManager>>,
    pub saved_search_manager: Arc<Mutex<SavedSearchManager>>,
    pub undo_stack: Arc<Mutex<UndoStack>>,
    pub batch_renamer: Arc<Mutex<BatchRenamer>>,
    pub batch_metadata_editor: Arc<Mutex<BatchMetadataEditor>>,
    pub pipeline_manager: Arc<Mutex<PipelineManager>>,
    pub macro_recorder: Arc<Mutex<MacroRecorder>>,
    pub macro_playback: Arc<Mutex<MacroPlayback>>,
    pub notification_manager: Arc<Mutex<NotificationManager>>,
    pub maintenance_engine: Arc<Mutex<MaintenanceEngine>>,
    pub db_connection: Arc<Mutex<rusqlite::Connection>>,
    pub migration_runner: Arc<MigrationRunner>,
    pub plugin_registry: Arc<Mutex<PluginRegistry>>,
    pub diagnostic_collector: Arc<Mutex<DiagnosticCollector>>,
    pub backup_manager: Arc<Mutex<BackupManager>>,
    pub app_handle: tauri::AppHandle,
}

impl AppState {
    pub fn new(app: &tauri::AppHandle) -> Self {
        let persistence = Persistence::new(app);
        let prefs = persistence.load_preferences();

        let metadata_map = persistence.load_metadata();
        let mut store = MetadataStore::new();
        for (_, meta) in metadata_map {
            store.insert(meta);
        }

        let mut basket_manager = BasketManager::new();
        for basket in persistence.load_baskets() {
            basket_manager.create(basket);
        }

        let app_dir: PathBuf = app.path_resolver()
            .app_data_dir()
            .unwrap_or_else(|| PathBuf::from("."));

        let db_path = app_dir.join("evoury.db");
        let db_conn = atlas_db::connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))
            .unwrap();
        let migration_runner = MigrationRunner;
        let migration_runner_ref = Arc::new(migration_runner);
        if let Err(e) = MigrationRunner::run(&db_conn) {
            eprintln!("Migration error: {}", e);
        }

        let plugin_registry = PluginRegistry::new();
        let diagnostic_collector = DiagnosticCollector::new("0.1.0", app_dir.clone());
        let backup_manager = BackupManager::new(app_dir.join("backups"));

        let workspace_path = app_dir.join("workspace.json");
        let snapshots_dir = app_dir.join("snapshots");
        let profiles_dir = app_dir.join("profiles");

        let workspace_manager = WorkspaceManager::new(workspace_path);
        let session_recovery = SessionRecovery::new(snapshots_dir);
        let scheduler = Scheduler::new(4);

        let event_bus = Arc::new(EventBus::new());

        let cache_dir = app_dir.join("cache");
        let _ = std::fs::create_dir_all(&cache_dir);
        let _ = std::fs::create_dir_all(&profiles_dir);

        let mut registry = Registry::new();
        registry.register_capability(atlas_core::Capability::IncrementalScan);
        registry.register_capability(atlas_core::Capability::ThumbnailCache);
        registry.register_capability(atlas_core::Capability::BackgroundWorkers);
        registry.register_capability(atlas_core::Capability::LargeLibrary);
        registry.register_capability(atlas_core::Capability::FileVersionDetection);
        registry.register_capability(atlas_core::Capability::DuplicateDetection);
        registry.register_capability(atlas_core::Capability::SmartCollections);
        registry.register_capability(atlas_core::Capability::VersionHistory);
        registry.register_capability(atlas_core::Capability::LayoutProfiles);
        registry.register_capability(atlas_core::Capability::WorkstationPresets);
        registry.register_capability(atlas_core::Capability::ViewProfiles);
        registry.register_capability(atlas_core::Capability::MultiWorkspace);
        registry.register_capability(atlas_core::Capability::DockableLayout);
        registry.register_capability(atlas_core::Capability::ContextMenu);
        registry.register_capability(atlas_core::Capability::QuickActions);
        registry.register_capability(atlas_core::Capability::SelectionEngine);
        registry.register_capability(atlas_core::Capability::NavigationHistory);
        registry.register_capability(atlas_core::Capability::Breadcrumbs);
        registry.register_capability(atlas_core::Capability::Tabs);
        registry.register_capability(atlas_core::Capability::KeyboardShortcuts);
        registry.register_capability(atlas_core::Capability::FocusMode);
        registry.register_capability(atlas_core::Capability::AnimationSystem);
        registry.register_capability(atlas_core::Capability::SpecializedViewers);
        registry.register_capability(atlas_core::Capability::CompareMode);
        registry.register_capability(atlas_core::Capability::ProgressiveLoading);
        registry.register_capability(atlas_core::Capability::GpuResourceManagement);
        registry.register_capability(atlas_core::Capability::ColorManagement);
        registry.register_capability(atlas_core::Capability::Tags);
        registry.register_capability(atlas_core::Capability::Ratings);
        registry.register_capability(atlas_core::Capability::Notes);
        registry.register_capability(atlas_core::Capability::CustomFields);
        registry.register_capability(atlas_core::Capability::Relationships);
        registry.register_capability(atlas_core::Capability::Projects);

        for provider in Self::create_providers() {
            registry.register_provider(provider);
        }

        let mut worker_pool = WorkerPool::new(2);
        worker_pool.register(Box::new(atlas_worker::ThumbnailWorker));
        worker_pool.register(Box::new(atlas_worker::DuplicateWorker));
        worker_pool.register(Box::new(atlas_worker::VersionWorker));
        worker_pool.register(Box::new(atlas_worker::CollectionWorker));

        let mut preview_manager = PreviewManager::new();
        preview_manager.providers.register(Box::new(atlas_preview::DefaultImageProvider));

        let library_count = 0;
        let large_config = if library_count > 100_000 {
            LargeLibraryConfig::xlarge()
        } else if library_count > 50_000 {
            LargeLibraryConfig::large()
        } else if library_count > 10_000 {
            LargeLibraryConfig::medium()
        } else {
            LargeLibraryConfig::small()
        };

        Self {
            library: Arc::new(Library::new()),
            metadata_store: Arc::new(Mutex::new(store)),
            basket_manager: Arc::new(Mutex::new(basket_manager)),
            event_bus: event_bus.clone(),
            watcher: Arc::new(Mutex::new(FilesystemWatcher::new())),
            persistence: Arc::new(Mutex::new(persistence)),
            preferences: Arc::new(Mutex::new(PreferenceStore::new(prefs))),
            registry: Arc::new(Mutex::new(registry)),
            scheduler: Arc::new(scheduler),
            sleep_manager: Arc::new(Mutex::new(SleepManager::new())),
            workspace_manager: Arc::new(Mutex::new(workspace_manager)),
            session_recovery: Arc::new(Mutex::new(session_recovery)),
            command_registry: Arc::new(Mutex::new(CommandRegistry::new())),
            command_executor: Arc::new(CommandExecutor::new()),
            cache_manager: Arc::new(Mutex::new(CacheManager::new(cache_dir))),
            worker_pool: Arc::new(Mutex::new(worker_pool)),
            duplicate_detector: Arc::new(DuplicateDetector::new()),
            collection_manager: Arc::new(Mutex::new(CollectionManager::new())),
            version_detector: Arc::new(VersionDetector::new()),
            large_library_config: large_config,
            layout_registry: Arc::new(Mutex::new(LayoutRegistry::new())),
            workstation_registry: Arc::new(Mutex::new(WorkstationRegistry::new())),
            profile_manager: Arc::new(Mutex::new(ProfileManager::new())),
            selection_engine: Arc::new(Mutex::new(SelectionEngine::new())),
            nav_history: Arc::new(Mutex::new(NavigationHistory::new(50))),
            tab_manager: Arc::new(Mutex::new(TabManager::new())),
            breadcrumb_trail: Arc::new(Mutex::new(BreadcrumbTrail::new())),
            shortcut_registry: Arc::new(Mutex::new(ShortcutRegistry::new())),
            animation_manager: Arc::new(Mutex::new(AnimationManager::new())),
            preview_manager: Arc::new(preview_manager),
            viewer_registry: Arc::new(ViewerRegistry::new()),
            compare_engine: Arc::new(Mutex::new(CompareEngine::new(uuid::Uuid::nil(), uuid::Uuid::nil()))),
            gpu_tracker: Arc::new(Mutex::new(GpuTracker::new())),
            color_manager: Arc::new(Mutex::new(ColorManager::new())),
            tag_manager: Arc::new(Mutex::new(TagManager::new())),
            custom_field_manager: Arc::new(Mutex::new(CustomFieldManager::new())),
            template_manager: Arc::new(Mutex::new(TemplateManager::new())),
            auto_classifier: Arc::new(Mutex::new(AutoClassifier::new())),
            relationship_manager: Arc::new(Mutex::new(RelationshipManager::new())),
            dependency_graph: Arc::new(Mutex::new(DependencyGraph::new())),
            links_version_manager: Arc::new(Mutex::new(VersionManager::new())),
            activity_tracker: Arc::new(Mutex::new(ActivityTracker::new())),
            version_history: Arc::new(Mutex::new(VersionHistory::new())),
            project_manager: Arc::new(Mutex::new(ProjectManager::new())),
            saved_search_manager: Arc::new(Mutex::new(SavedSearchManager::new())),
            undo_stack: Arc::new(Mutex::new(UndoStack::new(100))),
            batch_renamer: Arc::new(Mutex::new(BatchRenamer::new())),
            batch_metadata_editor: Arc::new(Mutex::new(BatchMetadataEditor::new())),
            pipeline_manager: Arc::new(Mutex::new(PipelineManager::new())),
            macro_recorder: Arc::new(Mutex::new(MacroRecorder::new())),
            macro_playback: Arc::new(Mutex::new(MacroPlayback::new())),
            notification_manager: Arc::new(Mutex::new(NotificationManager::new())),
            maintenance_engine: Arc::new(Mutex::new(MaintenanceEngine::new())),
            db_connection: Arc::new(Mutex::new(db_conn)),
            migration_runner: migration_runner_ref,
            plugin_registry: Arc::new(Mutex::new(plugin_registry)),
            diagnostic_collector: Arc::new(Mutex::new(diagnostic_collector)),
            backup_manager: Arc::new(Mutex::new(backup_manager)),
            app_handle: app.app_handle(),
        }
    }

    fn create_providers() -> Vec<Provider> {
        vec![
            Provider::new("atlas-cache", ProviderKind::Cache, "Thumbnail Cache", "0.1.0"),
            Provider::new("atlas-worker", ProviderKind::Worker, "Background Workers", "0.1.0"),
            Provider::new("atlas-duplicate", ProviderKind::Capability, "Duplicate Detection", "0.1.0"),
            Provider::new("atlas-collection", ProviderKind::Capability, "Smart Collections", "0.1.0"),
            Provider::new("atlas-links", ProviderKind::Capability, "Version & Links", "0.1.0"),
            Provider::new("atlas-haystack", ProviderKind::Capability, "Large Library Support", "0.1.0"),
            Provider::new("atlas-layout", ProviderKind::Capability, "Layout Engine", "0.1.0"),
            Provider::new("atlas-workstation", ProviderKind::Workstation, "Workstation Presets", "0.1.0"),
            Provider::new("atlas-profiles", ProviderKind::Capability, "View/Preview/Search Profiles", "0.1.0"),
            Provider::new("atlas-selection", ProviderKind::Capability, "Selection Engine", "0.1.0"),
            Provider::new("atlas-navigation", ProviderKind::Capability, "Navigation History & Tabs", "0.1.0"),
            Provider::new("atlas-keyboard", ProviderKind::Capability, "Keyboard Shortcuts", "0.1.0"),
            Provider::new("atlas-animation", ProviderKind::Capability, "Animation System", "0.1.0"),
            Provider::new("atlas-viewer", ProviderKind::Preview, "Specialized Viewers", "0.1.0"),
            Provider::new("atlas-compare", ProviderKind::Capability, "Compare Mode", "0.1.0"),
            Provider::new("atlas-gpu", ProviderKind::Capability, "GPU Resource Manager", "0.1.0"),
            Provider::new("atlas-color", ProviderKind::Capability, "Color Management", "0.1.0"),
            Provider::new("atlas-preview", ProviderKind::Preview, "Preview Manager", "0.1.0"),
            Provider::new("atlas-metadata", ProviderKind::Capability, "Tags, Custom Fields & Templates", "0.1.0"),
            Provider::new("atlas-project", ProviderKind::Capability, "Project Management", "0.1.0"),
            Provider::new("atlas-history", ProviderKind::Capability, "Activity & Version History", "0.1.0"),
            Provider::new("atlas-links", ProviderKind::Capability, "Relationships & Dependencies", "0.1.0"),
        ]
    }

    pub fn start_scheduler(&self) {
        self.scheduler.start();
    }
}
