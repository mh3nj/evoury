use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGroup {
    pub id: Uuid,
    pub base_name: String,
    pub versions: Vec<VersionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub asset_id: Uuid,
    pub label: String,
    pub sort_key: u32,
}

pub struct VersionDetector;

impl VersionDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(assets: &[(Uuid, String)]) -> Vec<VersionGroup> {
        let mut groups: HashMap<String, Vec<(Uuid, String, u32)>> = HashMap::new();

        for (id, name) in assets {
            let stem = std::path::Path::new(name)
                .file_stem()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();

            if let Some((base, label, sort_key)) = Self::parse_version(&stem) {
                groups.entry(base).or_default().push((*id, label, sort_key));
            }
        }

        groups
            .into_iter()
            .filter(|(_, versions)| versions.len() >= 2)
            .map(|(base_name, mut versions)| {
                versions.sort_by_key(|(_, _, k)| *k);
                VersionGroup {
                    id: Uuid::new_v4(),
                    base_name,
                    versions: versions.into_iter().map(|(id, label, _)| VersionEntry {
                        asset_id: id,
                        label,
                        sort_key: 0,
                    }).collect(),
                }
            })
            .collect()
    }

    fn parse_version(stem: &str) -> Option<(String, String, u32)> {
        let lower = stem.to_lowercase();

        // Pattern: name_v1, name_v2, name_v3
        if let Some(pos) = lower.rfind("_v") {
            let suffix = &lower[pos + 2..];
            if let Ok(num) = suffix.parse::<u32>() {
                let base = stem[..pos].to_string();
                return Some((base, format!("v{}", num), num));
            }
        }

        // Pattern: name-final
        if lower.ends_with("_final") {
            let base = stem[..stem.len() - 6].to_string();
            return Some((base, "final".into(), 999));
        }

        // Pattern: name-01, name-02
        if let Some(pos) = lower.rfind('-') {
            let suffix = &lower[pos + 1..];
            if suffix.len() == 2 && suffix.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(num) = suffix.parse::<u32>() {
                    let base = stem[..pos].to_string();
                    return Some((base, format!("{:02}", num), num));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_v1_v2() {
        let assets = vec![
            (Uuid::new_v4(), "logo_v1.ai".into()),
            (Uuid::new_v4(), "logo_v2.ai".into()),
            (Uuid::new_v4(), "logo_v3.ai".into()),
        ];
        let groups = VersionDetector::detect(&assets);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].versions.len(), 3);
    }

    #[test]
    fn test_final_version() {
        let assets = vec![
            (Uuid::new_v4(), "poster_final.ai".into()),
            (Uuid::new_v4(), "poster_v1.ai".into()),
        ];
        let groups = VersionDetector::detect(&assets);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].versions.len(), 2);
    }

    #[test]
    fn test_no_version_single_file() {
        let assets = vec![
            (Uuid::new_v4(), "logo.ai".into()),
        ];
        let groups = VersionDetector::detect(&assets);
        assert_eq!(groups.len(), 0);
    }
}
