mod commands;
mod event_bridge;
mod persistence;
mod presentation;
mod state;

use commands::tray::CLOSE_TO_TRAY;
use state::AppState;
use std::sync::atomic::Ordering;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub fn run() {
    let show = CustomMenuItem::new("show".to_string(), "Show Evoury");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .setup(|app| {
            let app_state = AppState::new(&app.handle());
            app.manage(app_state);

            let state = app.state::<AppState>();
            event_bridge::spawn_event_forwarder(app.handle(), &state.event_bus);
            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                if CLOSE_TO_TRAY.load(Ordering::SeqCst) {
                    let _ = event.window().hide();
                    api.prevent_close();
                }
            }
        })
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "show" => {
                        if let Some(window) = app.get_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::library::scan_library,
            commands::library::asset_count,
            commands::library::get_assets,
            commands::library::start_watching,
            commands::library::stop_watching,
            commands::library::is_watching,
            commands::search::search_assets,
            commands::search::suggest_assets,
            commands::search::get_asset_batch,
            commands::metadata::get_metadata,
            commands::metadata::update_metadata,
            commands::metadata::set_tags,
            commands::metadata::set_rating,
            commands::metadata::set_favorite,
            commands::metadata::set_notes,
            commands::preferences::get_preferences,
            commands::preferences::set_preferences,
            commands::basket::create_basket,
            commands::basket::get_baskets,
            commands::basket::add_to_basket,
            commands::basket::remove_from_basket,
            commands::basket::clear_basket,
            commands::basket::delete_basket,
            commands::basket::is_in_basket,
            commands::system::greet,
            commands::system::get_version,
            commands::system::get_diagnostics,
            commands::system::get_power_mode,
            commands::system::touch_activity,
            commands::system::save_session,
            commands::system::recover_session,
            commands::system::open_external_file,
            commands::system::open_file_location,
            commands::tags::add_tag,
            commands::tags::remove_tag,
            commands::tags::get_tag,
            commands::tags::get_all_tags,
            commands::tags::update_tag,
            commands::tags::move_tag,
            commands::collections::get_collections,
            commands::collections::add_collection,
            commands::collections::remove_collection,
            commands::collections::get_collection_children,
            commands::collections::move_collection,
            commands::collections::evaluate_smart_collections,
            commands::projects::create_project,
            commands::projects::get_project,
            commands::projects::get_all_projects,
            commands::projects::remove_project,
            commands::projects::add_asset_to_project,
            commands::projects::remove_asset_from_project,
            commands::projects::get_project_assets,
            commands::relationships::add_relationship,
            commands::relationships::remove_relationship,
            commands::relationships::get_relationships_for_asset,
            commands::relationships::get_related_assets,
            commands::relationships::get_dependency_chain,
            commands::relationships::get_dependent_chain,
            commands::activity::record_activity,
            commands::activity::get_asset_activity,
            commands::activity::get_recent_activity,
            commands::activity::get_most_viewed,
            commands::versions::commit_version,
            commands::versions::get_version_history,
            commands::versions::restore_version,
            commands::versions::delete_version,
            commands::versions::get_latest_version,
            commands::saved_searches::save_search,
            commands::saved_searches::get_saved_searches,
            commands::saved_searches::remove_saved_search,
            commands::saved_searches::create_search_folder,
            commands::saved_searches::get_search_folders,
            commands::saved_searches::move_search_to_folder,
            commands::templates::add_template,
            commands::templates::get_templates,
            commands::templates::remove_template,
            commands::auto_classify::add_classify_rule,
            commands::auto_classify::get_classify_rules,
            commands::auto_classify::remove_classify_rule,
            commands::auto_classify::classify_asset,
            commands::workflow::push_undo,
            commands::workflow::undo,
            commands::workflow::redo,
            commands::workflow::can_undo,
            commands::workflow::can_redo,
            commands::workflow::batch_rename_preview,
            commands::workflow::batch_rename_apply,
            commands::workflow::batch_set_tags,
            commands::workflow::batch_set_rating,
            commands::workflow::batch_set_favorite,
            commands::workflow::create_pipeline,
            commands::workflow::get_pipelines,
            commands::workflow::remove_pipeline,
            commands::workflow::add_pipeline_stage,
            commands::workflow::start_macro_recording,
            commands::workflow::stop_macro_recording,
            commands::workflow::record_macro_step,
            commands::workflow::is_recording,
            commands::workflow::get_macros,
            commands::workflow::remove_macro,
            commands::workflow::get_notifications,
            commands::workflow::get_unread_notifications,
            commands::workflow::get_unread_count,
            commands::workflow::mark_notification_read,
            commands::workflow::mark_all_notifications_read,
            commands::workflow::dismiss_notification,
            commands::workflow::clear_notifications,
            commands::workflow::run_maintenance,
            commands::workflow::run_all_maintenance,
            commands::workflow::get_maintenance_reports,
            commands::workflow::get_maintenance_schedule,
            commands::workflow::set_maintenance_schedule,
            commands::platform::get_db_migrations,
            commands::platform::get_db_pending_migrations,
            commands::platform::get_plugins,
            commands::platform::unregister_plugin,
            commands::platform::get_crash_reports,
            commands::platform::get_system_info,
            commands::platform::export_diagnostic_bundle,
            commands::platform::report_crash,
            commands::platform::list_backups,
            commands::platform::create_backup,
            commands::platform::restore_backup,
            commands::tray::set_close_to_tray,
            commands::tray::set_tray_on_start,
            commands::tray::set_autostart,
            commands::tray::hide_main_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Evoury");
}
