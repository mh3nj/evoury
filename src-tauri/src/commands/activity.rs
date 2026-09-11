use tauri::State;
use uuid::Uuid;
use atlas_core::{ActivityEvent, ActivityEventType};
use crate::state::AppState;

#[tauri::command]
pub fn record_activity(state: State<'_, AppState>, asset_id: String, event_type: String, details: String) -> Result<ActivityEvent, String> {
    let id = Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    let etype = parse_event_type(&event_type).ok_or("Invalid event type")?;
    state.activity_tracker.lock()
        .map_err(|e| e.to_string())
        .map(|mut t| t.record_event(id, etype, &details))
}

#[tauri::command]
pub fn get_asset_activity(state: State<'_, AppState>, asset_id: String, limit: usize) -> Vec<ActivityEvent> {
    let id = Uuid::parse_str(&asset_id).unwrap_or_default();
    state.activity_tracker.lock()
        .map(|t| t.get_events_for_asset(&id, limit).into_iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn get_recent_activity(state: State<'_, AppState>, hours: i64) -> Vec<ActivityEvent> {
    state.activity_tracker.lock()
        .map(|t| t.recent_activity(hours).into_iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn get_most_viewed(state: State<'_, AppState>, limit: usize) -> Vec<(String, usize)> {
    state.activity_tracker.lock()
        .map(|t| t.most_viewed(limit).into_iter().map(|(id, c)| (id.to_string(), c)).collect())
        .unwrap_or_default()
}

fn parse_event_type(s: &str) -> Option<ActivityEventType> {
    match s.to_lowercase().as_str() {
        "view" => Some(ActivityEventType::View),
        "open" => Some(ActivityEventType::Open),
        "export" => Some(ActivityEventType::Export),
        "edit" => Some(ActivityEventType::Edit),
        "tagadd" | "tag_add" | "tagadded" => Some(ActivityEventType::TagAdd),
        "tagremove" | "tag_remove" | "tagremoved" => Some(ActivityEventType::TagRemove),
        "ratingchange" | "rating_change" => Some(ActivityEventType::RatingChange),
        "favoritetoggle" | "favorite_toggle" => Some(ActivityEventType::FavoriteToggle),
        "noteupdate" | "note_update" => Some(ActivityEventType::NoteUpdate),
        "metadatachange" | "metadata_change" => Some(ActivityEventType::MetadataChange),
        "relationshipadd" | "relationship_add" => Some(ActivityEventType::RelationshipAdd),
        "relationshipremove" | "relationship_remove" => Some(ActivityEventType::RelationshipRemove),
        "versioncreate" | "version_create" => Some(ActivityEventType::VersionCreate),
        "versionrestore" | "version_restore" => Some(ActivityEventType::VersionRestore),
        _ => None,
    }
}
