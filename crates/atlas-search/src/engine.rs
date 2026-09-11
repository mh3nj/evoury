use atlas_core::Asset;
use atlas_metadata::AssetMetadata;
use crate::query::{SearchQuery, SortField};
use crate::result::SearchResult;

pub struct SearchEngine;

impl SearchEngine {
    pub fn search(
        assets: &[Asset],
        metadata: &[AssetMetadata],
        query: &SearchQuery,
    ) -> Vec<SearchResult> {
        let meta_map: std::collections::HashMap<_, _> = metadata
            .iter()
            .map(|m| (m.asset_id, m))
            .collect();

        let mut results: Vec<SearchResult> = Vec::new();

        for asset in assets {
            let meta = meta_map.get(&asset.id);
            let score = Self::score_asset(asset, meta.copied(), query);

            if score < 0.0 {
                continue;
            }

            if let Some(m) = meta {
                if query.favorite_only && !m.favorite {
                    continue;
                }
                if m.rating < query.min_rating {
                    continue;
                }
            } else {
                if query.favorite_only || query.min_rating > 0 {
                    continue;
                }
            }

            results.push(SearchResult {
                asset_id: asset.id,
                score,
            });
        }

        Self::sort_results(&mut results, query);
        results
    }

    pub fn suggest(assets: &[Asset], prefix: &str, max: usize) -> Vec<String> {
        let prefix = prefix.to_lowercase();
        let mut suggestions: Vec<String> = assets
            .iter()
            .map(|a| a.name.to_lowercase())
            .filter(|n| n.contains(&prefix))
            .collect();
        suggestions.sort();
        suggestions.dedup();
        suggestions.truncate(max);
        suggestions
    }

    fn score_asset(asset: &Asset, meta: Option<&AssetMetadata>, query: &SearchQuery) -> f32 {
        let mut score = 0.0;

        if let Some(ref file_type) = query.file_type {
            let asset_type = format!("{:?}", asset.archive_type).to_lowercase();
            if !asset_type.contains(&file_type.to_lowercase()) {
                return -1.0;
            }
        }

        if let Some(ref collection) = query.collection {
            let path_lower = asset.archive_path.to_string_lossy().to_lowercase();
            if !path_lower.contains(&collection.to_lowercase()) {
                return -1.0;
            }
        }

        if let Some(ref category) = query.category {
            let path_lower = asset.archive_path.to_string_lossy().to_lowercase();
            if !path_lower.contains(&category.to_lowercase()) {
                return -1.0;
            }
        }

        for term in &query.text {
            let term_lower = term.to_lowercase();
            let name_lower = asset.name.to_lowercase();

            if name_lower == term_lower {
                score += 10.0;
            } else if name_lower.starts_with(&term_lower) {
                score += 5.0;
            } else if name_lower.contains(&term_lower) {
                score += 2.0;
            }

            if let Some(m) = meta {
                for tag in &m.tags {
                    if tag.to_lowercase().contains(&term_lower) {
                        score += 3.0;
                    }
                }
                if m.notes.to_lowercase().contains(&term_lower) {
                    score += 1.0;
                }
            }

            let path_lower = asset.archive_path.to_string_lossy().to_lowercase();
            if path_lower.contains(&term_lower) {
                score += 0.5;
            }
        }

        if let Some(m) = meta {
            score += m.rating as f32 * 2.0;
            if m.favorite {
                score += 5.0;
            }
        }

        score
    }

    fn sort_results(results: &mut Vec<SearchResult>, query: &SearchQuery) {
        match query.sort {
            SortField::Score => {
                if query.sort_desc {
                    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
                } else {
                    results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
                }
            }
            _ => {
                if query.sort_desc {
                    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
                } else {
                    results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
                }
            }
        }
    }
}
