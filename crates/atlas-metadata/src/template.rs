use crate::metadata::AssetMetadata;
use atlas_core::MetadataTemplate;
use uuid::Uuid;

#[derive(Debug)]
pub struct TemplateManager {
    templates: Vec<MetadataTemplate>,
}

impl TemplateManager {
    pub fn new() -> Self {
        Self { templates: Vec::new() }
    }

    pub fn add_template(&mut self, template: MetadataTemplate) {
        self.templates.push(template);
    }

    pub fn remove_template(&mut self, id: &Uuid) {
        self.templates.retain(|t| t.id != *id);
    }

    pub fn get_template(&self, id: &Uuid) -> Option<&MetadataTemplate> {
        self.templates.iter().find(|t| t.id == *id)
    }

    pub fn all_templates(&self) -> Vec<&MetadataTemplate> {
        self.templates.iter().collect()
    }

    pub fn apply_template(template: &MetadataTemplate, metadata: &mut AssetMetadata) {
        for tag in &template.tags {
            if !metadata.tags.contains(tag) {
                metadata.tags.push(tag.clone());
            }
        }
        for field in &template.custom_fields {
            metadata.custom_fields.insert(field.name.clone(), field.value.clone());
        }
        if !template.notes_template.is_empty() {
            metadata.notes = template.notes_template.clone();
        }
        if let Some(rating) = template.rating {
            metadata.rating = rating;
        }
        if let Some(favorite) = template.favorite {
            metadata.favorite = favorite;
        }
    }
}
