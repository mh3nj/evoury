use atlas_core::{SavedSearch, SearchFolder};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SavedSearchManager {
    searches: Vec<SavedSearch>,
    folders: Vec<SearchFolder>,
}

impl SavedSearchManager {
    pub fn new() -> Self {
        Self {
            searches: Vec::new(),
            folders: Vec::new(),
        }
    }

    pub fn save_search(&mut self, name: &str, query_text: &str) -> SavedSearch {
        let search = SavedSearch::new(name, query_text);
        self.searches.push(search.clone());
        search
    }

    pub fn get_search(&self, id: &Uuid) -> Option<&SavedSearch> {
        self.searches.iter().find(|s| s.id == *id)
    }

    pub fn remove_search(&mut self, id: &Uuid) {
        self.searches.retain(|s| s.id != *id);
    }

    pub fn update_search(
        &mut self,
        id: &Uuid,
        name: Option<&str>,
        query_text: Option<&str>,
        icon: Option<&str>,
    ) -> Result<(), String> {
        let search = self.searches.iter_mut().find(|s| s.id == *id);
        match search {
            Some(s) => {
                if let Some(n) = name {
                    s.name = n.to_string();
                }
                if let Some(q) = query_text {
                    s.query_text = q.to_string();
                }
                if let Some(i) = icon {
                    s.icon = i.to_string();
                }
                Ok(())
            }
            None => Err(format!("Search with id {} not found", id)),
        }
    }

    pub fn all_searches(&self) -> Vec<&SavedSearch> {
        self.searches.iter().collect()
    }

    pub fn searches_in_folder(&self, folder_id: &Uuid) -> Vec<&SavedSearch> {
        self.searches
            .iter()
            .filter(|s| s.folder_id == Some(*folder_id))
            .collect()
    }

    pub fn move_search_to_folder(
        &mut self,
        search_id: &Uuid,
        folder_id: Option<Uuid>,
    ) -> Result<(), String> {
        let search = self.searches.iter_mut().find(|s| s.id == *search_id);
        match search {
            Some(s) => {
                s.folder_id = folder_id;
                Ok(())
            }
            None => Err(format!("Search with id {} not found", search_id)),
        }
    }

    pub fn create_folder(&mut self, name: &str) -> SearchFolder {
        let folder = SearchFolder::new(name);
        self.folders.push(folder.clone());
        folder
    }

    pub fn create_subfolder(&mut self, parent_id: &Uuid, name: &str) -> SearchFolder {
        let mut folder = SearchFolder::new(name);
        folder.parent_id = Some(*parent_id);
        self.folders.push(folder.clone());
        folder
    }

    pub fn get_folder(&self, id: &Uuid) -> Option<&SearchFolder> {
        self.folders.iter().find(|f| f.id == *id)
    }

    pub fn remove_folder(&mut self, id: &Uuid) -> Result<(), String> {
        if self.folders.iter().any(|f| f.id == *id) {
            for search in self.searches.iter_mut() {
                if search.folder_id == Some(*id) {
                    search.folder_id = None;
                }
            }
            self.folders.retain(|f| f.id != *id);
            Ok(())
        } else {
            Err(format!("Folder with id {} not found", id))
        }
    }

    pub fn all_folders(&self) -> Vec<&SearchFolder> {
        self.folders.iter().collect()
    }

    pub fn subfolders(&self, parent_id: &Uuid) -> Vec<&SearchFolder> {
        self.folders
            .iter()
            .filter(|f| f.parent_id == Some(*parent_id))
            .collect()
    }

    pub fn update_folder(
        &mut self,
        id: &Uuid,
        name: Option<&str>,
        icon: Option<&str>,
        color: Option<&str>,
    ) -> Result<(), String> {
        let folder = self.folders.iter_mut().find(|f| f.id == *id);
        match folder {
            Some(f) => {
                if let Some(n) = name {
                    f.name = n.to_string();
                }
                if let Some(i) = icon {
                    f.icon = i.to_string();
                }
                if let Some(c) = color {
                    f.color = Some(c.to_string());
                }
                Ok(())
            }
            None => Err(format!("Folder with id {} not found", id)),
        }
    }

    pub fn find_by_query_text(&self, text: &str) -> Option<&SavedSearch> {
        self.searches.iter().find(|s| s.query_text == text)
    }

    pub fn search_saved(&self, query: &str) -> Vec<&SavedSearch> {
        let lower_query = query.to_lowercase();
        self.searches
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&lower_query))
            .collect()
    }
}
