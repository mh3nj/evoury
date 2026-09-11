use std::collections::HashMap;
use uuid::Uuid;
use atlas_core::{Project, ProjectNote, Deliverable, DeliverableStatus};

pub struct ProjectManager {
    projects: HashMap<Uuid, Project>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    // ── CRUD ───────────────────────────────────────────────

    pub fn create_project(&mut self, name: &str) -> Project {
        let project = Project::new(name);
        let id = project.id;
        self.projects.insert(id, project.clone());
        project
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.insert(project.id, project);
    }

    pub fn get_project(&self, id: &Uuid) -> Option<&Project> {
        self.projects.get(id)
    }

    pub fn get_project_mut(&mut self, id: &Uuid) -> Option<&mut Project> {
        self.projects.get_mut(id)
    }

    pub fn remove_project(&mut self, id: &Uuid) -> Result<(), String> {
        if self.projects.remove(id).is_some() {
            Ok(())
        } else {
            Err(format!("Project with id {id} not found"))
        }
    }

    pub fn update_project(
        &mut self,
        id: &Uuid,
        name: Option<&str>,
        description: Option<&str>,
        icon: Option<&str>,
        color: Option<&str>,
    ) -> Result<(), String> {
        let project = self.projects.get_mut(id).ok_or_else(|| format!("Project with id {id} not found"))?;
        if let Some(v) = name {
            project.name = v.to_string();
        }
        if let Some(v) = description {
            project.description = v.to_string();
        }
        if let Some(v) = icon {
            project.icon = v.to_string();
        }
        if let Some(v) = color {
            project.color = Some(v.to_string());
        }
        project.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn all_projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    // ── Asset management ───────────────────────────────────

    pub fn add_asset_to_project(&mut self, project_id: &Uuid, asset_id: Uuid) -> Result<(), String> {
        let project = self.projects.get_mut(project_id).ok_or_else(|| format!("Project with id {project_id} not found"))?;
        project.asset_ids.push(asset_id);
        project.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn remove_asset_from_project(&mut self, project_id: &Uuid, asset_id: &Uuid) -> Result<(), String> {
        let project = self.projects.get_mut(project_id).ok_or_else(|| format!("Project with id {project_id} not found"))?;
        let len_before = project.asset_ids.len();
        project.asset_ids.retain(|id| id != asset_id);
        if project.asset_ids.len() == len_before {
            return Err(format!("Asset with id {asset_id} not found in project"));
        }
        project.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn assets_in_project(&self, project_id: &Uuid) -> Vec<Uuid> {
        self.projects
            .get(project_id)
            .map(|p| p.asset_ids.clone())
            .unwrap_or_default()
    }

    pub fn find_project_for_asset(&self, asset_id: &Uuid) -> Option<&Project> {
        self.projects.values().find(|p| p.asset_ids.contains(asset_id))
    }

    // ── Notes ──────────────────────────────────────────────

    pub fn add_note(&mut self, project_id: &Uuid, title: &str, content: &str) -> Result<ProjectNote, String> {
        let project = self.projects.get_mut(project_id).ok_or_else(|| format!("Project with id {project_id} not found"))?;
        let note = ProjectNote {
            id: Uuid::new_v4(),
            title: title.to_string(),
            content: content.to_string(),
            pinned: false,
            created_at: chrono::Utc::now(),
        };
        project.notes.push(note.clone());
        project.updated_at = chrono::Utc::now();
        Ok(note)
    }

    pub fn update_note(
        &mut self,
        project_id: &Uuid,
        note_id: &Uuid,
        title: Option<&str>,
        content: Option<&str>,
        pinned: Option<bool>,
    ) -> Result<(), String> {
        let project = self.projects.get_mut(project_id).ok_or_else(|| format!("Project with id {project_id} not found"))?;
        let note = project
            .notes
            .iter_mut()
            .find(|n| n.id == *note_id)
            .ok_or_else(|| format!("Note with id {note_id} not found"))?;
        if let Some(v) = title {
            note.title = v.to_string();
        }
        if let Some(v) = content {
            note.content = v.to_string();
        }
        if let Some(v) = pinned {
            note.pinned = v;
        }
        project.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn remove_note(&mut self, project_id: &Uuid, note_id: &Uuid) -> Result<(), String> {
        let project = self.projects.get_mut(project_id).ok_or_else(|| format!("Project with id {project_id} not found"))?;
        let len_before = project.notes.len();
        project.notes.retain(|n| n.id != *note_id);
        if project.notes.len() == len_before {
            return Err(format!("Note with id {note_id} not found"));
        }
        project.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn notes_in_project(&self, project_id: &Uuid) -> Vec<&ProjectNote> {
        self.projects
            .get(project_id)
            .map(|p| p.notes.iter().collect())
            .unwrap_or_default()
    }

    // ── Deliverables ───────────────────────────────────────

    pub fn add_deliverable(&mut self, project_id: &Uuid, name: &str, description: &str) -> Result<Deliverable, String> {
        let project = self.projects.get_mut(project_id).ok_or_else(|| format!("Project with id {project_id} not found"))?;
        let deliverable = Deliverable {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            path: None,
            asset_id: None,
            status: DeliverableStatus::Planned,
            due_date: None,
        };
        project.deliverables.push(deliverable.clone());
        project.updated_at = chrono::Utc::now();
        Ok(deliverable)
    }

    pub fn update_deliverable_status(
        &mut self,
        project_id: &Uuid,
        deliverable_id: &Uuid,
        status: DeliverableStatus,
    ) -> Result<(), String> {
        let project = self.projects.get_mut(project_id).ok_or_else(|| format!("Project with id {project_id} not found"))?;
        let d = project
            .deliverables
            .iter_mut()
            .find(|d| d.id == *deliverable_id)
            .ok_or_else(|| format!("Deliverable with id {deliverable_id} not found"))?;
        d.status = status;
        project.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn link_deliverable_to_asset(
        &mut self,
        project_id: &Uuid,
        deliverable_id: &Uuid,
        asset_id: Uuid,
    ) -> Result<(), String> {
        let project = self.projects.get_mut(project_id).ok_or_else(|| format!("Project with id {project_id} not found"))?;
        let d = project
            .deliverables
            .iter_mut()
            .find(|d| d.id == *deliverable_id)
            .ok_or_else(|| format!("Deliverable with id {deliverable_id} not found"))?;
        d.asset_id = Some(asset_id);
        project.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn remove_deliverable(&mut self, project_id: &Uuid, deliverable_id: &Uuid) -> Result<(), String> {
        let project = self.projects.get_mut(project_id).ok_or_else(|| format!("Project with id {project_id} not found"))?;
        let len_before = project.deliverables.len();
        project.deliverables.retain(|d| d.id != *deliverable_id);
        if project.deliverables.len() == len_before {
            return Err(format!("Deliverable with id {deliverable_id} not found"));
        }
        project.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn deliverables_in_project(&self, project_id: &Uuid) -> Vec<&Deliverable> {
        self.projects
            .get(project_id)
            .map(|p| p.deliverables.iter().collect())
            .unwrap_or_default()
    }

    pub fn deliverables_by_status(&self, project_id: &Uuid, status: &DeliverableStatus) -> Vec<&Deliverable> {
        self.projects
            .get(project_id)
            .map(|p| p.deliverables.iter().filter(|d| d.status == *status).collect())
            .unwrap_or_default()
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}
