pub struct BatchMetadataEditor;

impl BatchMetadataEditor {
    pub fn new() -> Self { Self }

    pub fn set_tags_batch(&self, asset_ids: &[uuid::Uuid], add_tags: &[String], remove_tags: &[String], existing_tags: &std::collections::HashMap<uuid::Uuid, Vec<String>>) -> Vec<(uuid::Uuid, Vec<String>)> {
        asset_ids.iter().map(|id| {
            let mut tags = existing_tags.get(id).cloned().unwrap_or_default();
            for t in add_tags { if !tags.contains(t) { tags.push(t.clone()); } }
            tags.retain(|t| !remove_tags.contains(t));
            (*id, tags)
        }).collect()
    }

    pub fn set_rating_batch(&self, asset_ids: &[uuid::Uuid], rating: u8) -> Vec<(uuid::Uuid, u8)> {
        let r = rating.min(5);
        asset_ids.iter().map(|id| (*id, r)).collect()
    }

    pub fn set_favorite_batch(&self, asset_ids: &[uuid::Uuid], favorite: bool) -> Vec<(uuid::Uuid, bool)> {
        asset_ids.iter().map(|id| (*id, favorite)).collect()
    }

    pub fn add_notes_batch(&self, asset_ids: &[uuid::Uuid], note_template: &str, existing_notes: &std::collections::HashMap<uuid::Uuid, String>) -> Vec<(uuid::Uuid, String)> {
        asset_ids.iter().map(|id| {
            let existing = existing_notes.get(id).cloned().unwrap_or_default();
            let new_note = if existing.is_empty() { note_template.to_string() } else { format!("{}\n{}", existing, note_template) };
            (*id, new_note)
        }).collect()
    }
}
