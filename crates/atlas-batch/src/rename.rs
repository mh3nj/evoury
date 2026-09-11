use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenamePattern {
    Counter { prefix: String, start: u32, digits: u32, suffix: String },
    Date { format: String, suffix: String },
    Regex { pattern: String, replacement: String },
    Metadata { field: String, template: String },
    Custom { template: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenamePreview {
    pub asset_id: Uuid,
    pub original_name: String,
    pub new_name: String,
    pub extension: String,
}

pub struct BatchRenamer;

impl BatchRenamer {
    pub fn new() -> Self { Self }

    pub fn preview(&self, asset_ids: &[Uuid], asset_names: &[(Uuid, String)], pattern: &RenamePattern, custom_fields: &std::collections::HashMap<Uuid, std::collections::HashMap<String, String>>) -> Vec<RenamePreview> {
        let mut results = Vec::new();
        for (i, (asset_id, name)) in asset_names.iter().enumerate() {
            let (stem, ext) = self.split_name(name);
            let new_stem = self.apply_pattern(stem, &ext, pattern, i, asset_id, custom_fields);
            let new_name = format!("{}.{}", new_stem, ext);
            results.push(RenamePreview { asset_id: *asset_id, original_name: name.clone(), new_name, extension: ext });
        }
        results
    }

    pub fn apply(&self, previews: &[RenamePreview]) -> Result<Vec<(Uuid, String)>, String> {
        Ok(previews.iter().map(|p| (p.asset_id, p.new_name.clone())).collect())
    }

    fn split_name<'a>(&self, name: &'a str) -> (&'a str, String) {
        if let Some(dot) = name.rfind('.') {
            let ext = &name[dot+1..];
            let stem = &name[..dot];
            (stem, ext.to_lowercase())
        } else {
            (name, String::new())
        }
    }

    fn apply_pattern(&self, stem: &str, ext: &str, pattern: &RenamePattern, index: usize, _asset_id: &Uuid, custom_fields: &std::collections::HashMap<Uuid, std::collections::HashMap<String, String>>) -> String {
        match pattern {
            RenamePattern::Counter { prefix, start, digits, suffix } => {
                let num = start + index as u32;
                format!("{}{:0width$}{}", prefix, num, suffix, width = *digits as usize)
            }
            RenamePattern::Date { format, suffix } => {
                let now = chrono::Utc::now();
                let date_str = now.format(format).to_string();
                format!("{}_{}", date_str, suffix)
            }
            RenamePattern::Regex { pattern: pat, replacement } => {
                let lower = stem.to_lowercase();
                if lower.contains(&pat.to_lowercase()) {
                    stem.replace(pat, replacement)
                } else {
                    stem.to_string()
                }
            }
            RenamePattern::Metadata { field, template } => {
                let fields = custom_fields.get(_asset_id);
                let val = fields.and_then(|f| f.get(field)).cloned().unwrap_or_default();
                template.replace(&format!("{{field:{}}}", field), &val)
            }
            RenamePattern::Custom { template } => {
                template
                    .replace("{counter}", &format!("{:04}", index + 1))
                    .replace("{name}", stem)
                    .replace("{ext}", ext)
            }
        }
    }
}
