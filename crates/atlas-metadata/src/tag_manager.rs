use atlas_core::HierarchicalTag;
use uuid::Uuid;

#[derive(Debug)]
pub struct TagManager {
    tags: Vec<HierarchicalTag>,
}

impl TagManager {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }

    pub fn add_tag(&mut self, tag: HierarchicalTag) -> Result<(), String> {
        if self.tags.iter().any(|t| t.name == tag.name && t.namespace == tag.namespace) {
            return Err(format!("Tag with name '{}' already exists in namespace '{}'", tag.name, tag.namespace));
        }
        self.tags.push(tag);
        Ok(())
    }

    pub fn remove_tag(&mut self, id: Uuid) -> Result<(), String> {
        let parent_id = self.tags.iter().find(|t| t.id == id).map(|t| t.parent_id);
        let parent_id = match parent_id {
            Some(pid) => pid,
            None => return Err("Tag not found".to_string()),
        };
        self.tags.retain(|t| t.id != id);
        for tag in self.tags.iter_mut() {
            if tag.parent_id == Some(id) {
                tag.parent_id = parent_id;
            }
        }
        Ok(())
    }

    pub fn get_tag(&self, id: &Uuid) -> Option<&HierarchicalTag> {
        self.tags.iter().find(|t| t.id == *id)
    }

    pub fn get_tag_by_name(&self, name: &str, namespace: &str) -> Option<&HierarchicalTag> {
        self.tags.iter().find(|t| t.name == name && t.namespace == namespace)
    }

    pub fn all_in_namespace(&self, namespace: &str) -> Vec<&HierarchicalTag> {
        self.tags.iter().filter(|t| t.namespace == namespace).collect()
    }

    pub fn children_of(&self, parent_id: &Uuid) -> Vec<&HierarchicalTag> {
        self.tags.iter().filter(|t| t.parent_id == Some(*parent_id)).collect()
    }

    pub fn ancestors(&self, id: &Uuid) -> Vec<Uuid> {
        let mut chain = Vec::new();
        let mut current = self.tags.iter().find(|t| t.id == *id);
        while let Some(tag) = current {
            match tag.parent_id {
                Some(parent_id) => {
                    chain.push(parent_id);
                    current = self.tags.iter().find(|t| t.id == parent_id);
                }
                None => break,
            }
        }
        chain
    }

    pub fn update_tag(&mut self, id: &Uuid, name: Option<&str>, color: Option<&str>, description: Option<&str>) -> Result<(), String> {
        let tag = self.tags.iter_mut().find(|t| t.id == *id).ok_or_else(|| "Tag not found".to_string())?;
        if let Some(n) = name {
            tag.name = n.to_string();
        }
        if let Some(c) = color {
            tag.color = Some(c.to_string());
        }
        if let Some(d) = description {
            tag.description = d.to_string();
        }
        Ok(())
    }

    pub fn move_tag(&mut self, id: &Uuid, new_parent_id: Option<Uuid>) -> Result<(), String> {
        if let Some(new_parent) = new_parent_id {
            if *id == new_parent {
                return Err("Cannot set a tag as its own parent".to_string());
            }
            let ancestors_of_new = self.ancestors(&new_parent);
            if ancestors_of_new.contains(id) {
                return Err("Moving would create a cycle".to_string());
            }
        }
        let tag = self.tags.iter_mut().find(|t| t.id == *id).ok_or_else(|| "Tag not found".to_string())?;
        tag.parent_id = new_parent_id;
        Ok(())
    }
}
