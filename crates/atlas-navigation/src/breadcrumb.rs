use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breadcrumb {
    pub label: String,
    pub path: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BreadcrumbTrail {
    pub crumbs: Vec<Breadcrumb>,
}

impl BreadcrumbTrail {
    pub fn new() -> Self {
        Self {
            crumbs: vec![Breadcrumb {
                label: "All Assets".into(),
                path: "/".into(),
                icon: Some("fa-home".into()),
            }],
        }
    }

    pub fn push(&mut self, label: &str, path: &str, icon: Option<&str>) {
        self.crumbs.push(Breadcrumb {
            label: label.to_string(),
            path: path.to_string(),
            icon: icon.map(|s| s.to_string()),
        });
    }

    pub fn pop(&mut self) -> Option<Breadcrumb> {
        if self.crumbs.len() > 1 {
            self.crumbs.pop()
        } else {
            None
        }
    }

    pub fn truncate(&mut self, to_index: usize) {
        if to_index + 1 < self.crumbs.len() {
            self.crumbs.truncate(to_index + 1);
        }
    }

    pub fn current(&self) -> Option<&Breadcrumb> {
        self.crumbs.last()
    }

    pub fn len(&self) -> usize {
        self.crumbs.len()
    }
}
