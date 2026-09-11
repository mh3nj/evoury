use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: Vec<String>,
    pub tags: Vec<String>,
    pub collection: Option<String>,
    pub category: Option<String>,
    pub file_type: Option<String>,
    pub favorite_only: bool,
    pub min_rating: u8,
    pub sort: SortField,
    pub sort_desc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortField {
    Name,
    Rating,
    Date,
    Size,
    Score,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            text: Vec::new(),
            tags: Vec::new(),
            collection: None,
            category: None,
            file_type: None,
            favorite_only: false,
            min_rating: 0,
            sort: SortField::Score,
            sort_desc: true,
        }
    }
}

pub fn parse_query(input: &str) -> SearchQuery {
    let mut query = SearchQuery::default();
    let tokens = tokenize(input);

    for token in tokens {
        if let Some((key, value)) = token.split_once(':') {
            match key.to_lowercase().as_str() {
                "tag" | "tags" => query.tags.push(value.to_string()),
                "collection" | "col" => query.collection = Some(value.to_string()),
                "category" | "cat" => query.category = Some(value.to_string()),
                "type" | "filetype" | "ext" => query.file_type = Some(value.to_string()),
                "favorite" | "fav" => query.favorite_only = value.eq_ignore_ascii_case("true") || value == "1",
                "rating" | "rate" | "stars" => {
                    if let Ok(r) = value.parse::<u8>() {
                        query.min_rating = r.min(5);
                    }
                }
                "sort" => match value.to_lowercase().as_str() {
                    "name" | "alpha" => { query.sort = SortField::Name; }
                    "rating" | "stars" => { query.sort = SortField::Rating; }
                    "date" | "modified" | "age" => { query.sort = SortField::Date; }
                    "size" => { query.sort = SortField::Size; }
                    "score" | "relevance" => { query.sort = SortField::Score; }
                    "asc" => { query.sort_desc = false; }
                    "desc" => { query.sort_desc = true; }
                    _ => {}
                },
                _ => query.text.push(token.to_string()),
            }
        } else {
            query.text.push(token.to_string());
        }
    }

    query
}

pub fn suggest_completions(input: &str) -> Vec<String> {
    let input = input.trim();
    if input.is_empty() {
        return vec![
            "type:".into(),
            "tag:".into(),
            "rating:".into(),
            "favorite:".into(),
            "collection:".into(),
            "category:".into(),
            "sort:".into(),
        ];
    }

    let fields = ["type:", "tag:", "rating:", "favorite:", "collection:", "category:", "sort:"];
    fields
        .iter()
        .filter(|f| f.starts_with(input) || input.starts_with(&f[..f.len().saturating_sub(1)]))
        .map(|f| {
            if let Some((_key, val)) = input.split_once(':') {
                if input.starts_with("type:") {
                    return format!("type:{}{}", val, ["zip", "rar", "7z", "tar", "avif", "png", "jpg"]
                        .iter()
                        .find(|s| s.starts_with(val))
                        .unwrap_or(&val));
                }
                if input.starts_with("tag:") || input.starts_with("tags:") {
                    return input.to_string();
                }
                if input.starts_with("rating:") || input.starts_with("rate:") {
                    for r in (1..=5).rev() {
                        if val.is_empty() || r.to_string().starts_with(val) {
                            return format!("rating:{}", r);
                        }
                    }
                    return input.to_string();
                }
                input.to_string()
            } else {
                f.to_string()
            }
        })
        .collect()
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;

    for ch in input.chars() {
        match ch {
            '"' => in_quote = !in_quote,
            ' ' if !in_quote => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_text() {
        let q = parse_query("billboard mockup");
        assert_eq!(q.text, vec!["billboard", "mockup"]);
    }

    #[test]
    fn test_parse_field_filters() {
        let q = parse_query("type:zip rating:5 favorite:true logo");
        assert_eq!(q.file_type, Some("zip".into()));
        assert_eq!(q.min_rating, 5);
        assert!(q.favorite_only);
        assert_eq!(q.text, vec!["logo"]);
    }

    #[test]
    fn test_parse_quoted_text() {
        let q = parse_query("\"luxury billboard\" type:rar");
        assert_eq!(q.text, vec!["luxury billboard"]);
        assert_eq!(q.file_type, Some("rar".into()));
    }

    #[test]
    fn test_suggestions() {
        let s = suggest_completions("t");
        assert!(s.contains(&"type:".to_string()));
        assert!(s.contains(&"tag:".to_string()));
    }
}
