use atlas_core::CustomFieldDefinition;
use uuid::Uuid;

#[derive(Debug)]
pub struct CustomFieldManager {
    definitions: Vec<CustomFieldDefinition>,
}

impl CustomFieldManager {
    pub fn new() -> Self {
        Self { definitions: Vec::new() }
    }

    pub fn add_definition(&mut self, def: CustomFieldDefinition) -> Result<(), String> {
        if self.definitions.iter().any(|d| d.name == def.name && d.category == def.category) {
            return Err(format!("Definition with name '{}' already exists in category '{}'", def.name, def.category));
        }
        self.definitions.push(def);
        Ok(())
    }

    pub fn remove_definition(&mut self, id: &Uuid) {
        self.definitions.retain(|d| d.id != *id);
    }

    pub fn get_definition(&self, id: &Uuid) -> Option<&CustomFieldDefinition> {
        self.definitions.iter().find(|d| d.id == *id)
    }

    pub fn all_definitions(&self) -> Vec<&CustomFieldDefinition> {
        self.definitions.iter().collect()
    }

    pub fn definitions_by_category(&self, category: &str) -> Vec<&CustomFieldDefinition> {
        self.definitions.iter().filter(|d| d.category == category).collect()
    }
}
