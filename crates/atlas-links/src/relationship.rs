use atlas_core::{Relationship, RelationshipKind};
use uuid::Uuid;

pub struct RelationshipManager {
    relationships: Vec<Relationship>,
}

impl RelationshipManager {
    pub fn new() -> Self {
        Self {
            relationships: Vec::new(),
        }
    }

    pub fn add_relationship(&mut self, rel: Relationship) -> Result<(), String> {
        if self.relationships.iter().any(|r| r.id == rel.id) {
            return Err(format!(
                "Relationship with id {} already exists",
                rel.id
            ));
        }
        self.relationships.push(rel);
        Ok(())
    }

    pub fn remove_relationship(&mut self, id: &Uuid) {
        self.relationships.retain(|r| r.id != *id);
    }

    pub fn get_relationship(&self, id: &Uuid) -> Option<&Relationship> {
        self.relationships.iter().find(|r| r.id == *id)
    }

    pub fn relationships_for_asset(&self, asset_id: &Uuid) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.source_id == *asset_id || r.target_id == *asset_id)
            .collect()
    }

    pub fn outgoing(&self, source_id: &Uuid) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.source_id == *source_id)
            .collect()
    }

    pub fn incoming(&self, target_id: &Uuid) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.target_id == *target_id)
            .collect()
    }

    pub fn related_asset_ids(&self, asset_id: &Uuid) -> Vec<Uuid> {
        let mut ids: Vec<Uuid> = self
            .relationships
            .iter()
            .filter(|r| r.source_id == *asset_id || r.target_id == *asset_id)
            .flat_map(|r| {
                if r.source_id == *asset_id {
                    vec![r.target_id]
                } else {
                    vec![r.source_id]
                }
            })
            .collect();
        ids.sort();
        ids.dedup();
        ids
    }

    pub fn by_kind(&self, kind: &RelationshipKind) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.kind == *kind)
            .collect()
    }
}
