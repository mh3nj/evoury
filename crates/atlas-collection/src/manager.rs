use std::collections::HashMap;
use uuid::Uuid;
use crate::collection::Collection;

pub struct CollectionManager {
    collections: HashMap<Uuid, Collection>,
}

impl CollectionManager {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
        }
    }

    pub fn add(&mut self, collection: Collection) {
        self.collections.insert(collection.id, collection);
    }

    pub fn get(&self, id: &Uuid) -> Option<&Collection> {
        self.collections.get(id)
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.collections.remove(id);
    }

    pub fn all(&self) -> Vec<Collection> {
        self.collections.values().cloned().collect()
    }

    pub fn add_child(&mut self, parent_id: Uuid, mut collection: Collection) -> Result<(), String> {
        if !self.collections.contains_key(&parent_id) {
            return Err(format!("Parent collection {} not found", parent_id));
        }
        collection.parent_id = Some(parent_id);
        self.collections.insert(collection.id, collection);
        Ok(())
    }

    pub fn children_of(&self, parent_id: Uuid) -> Vec<Collection> {
        self.collections
            .values()
            .filter(|c| c.parent_id == Some(parent_id))
            .cloned()
            .collect()
    }

    pub fn ancestors(&self, collection_id: Uuid) -> Vec<Collection> {
        let mut result = Vec::new();
        let mut current_id = collection_id;
        loop {
            let current = match self.collections.get(&current_id) {
                Some(c) => c,
                None => break,
            };
            match current.parent_id {
                Some(pid) => {
                    if let Some(parent) = self.collections.get(&pid) {
                        result.push(parent.clone());
                        current_id = pid;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        result
    }

    pub fn subtree(&self, collection_id: Uuid) -> Vec<Collection> {
        let mut result = Vec::new();
        let mut stack = vec![collection_id];
        while let Some(id) = stack.pop() {
            let children: Vec<Collection> = self
                .collections
                .values()
                .filter(|c| c.parent_id == Some(id))
                .cloned()
                .collect();
            for child in &children {
                stack.push(child.id);
            }
            result.extend(children);
        }
        result
    }

    pub fn move_to(&mut self, collection_id: Uuid, new_parent_id: Option<Uuid>) -> Result<(), String> {
        if !self.collections.contains_key(&collection_id) {
            return Err(format!("Collection {} not found", collection_id));
        }

        if let Some(new_parent) = new_parent_id {
            if !self.collections.contains_key(&new_parent) {
                return Err(format!("Target parent collection {} not found", new_parent));
            }
            if self.is_descendant_of(new_parent, collection_id) {
                return Err("Cannot move a collection into its own descendant".into());
            }
        }

        if let Some(collection) = self.collections.get_mut(&collection_id) {
            collection.parent_id = new_parent_id;
        }
        Ok(())
    }

    fn is_descendant_of(&self, target: Uuid, ancestor: Uuid) -> bool {
        let mut current = target;
        loop {
            match self.collections.get(&current) {
                Some(c) => match c.parent_id {
                    Some(pid) => {
                        if pid == ancestor {
                            return true;
                        }
                        current = pid;
                    }
                    None => return false,
                },
                None => return false,
            }
        }
    }
}
