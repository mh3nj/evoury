use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleField {
    Tag,
    Rating,
    Favorite,
    Type,
    Name,
    Notes,
    DateAdded,
    DateModified,
    FileSize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    EndsWith,
    InRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRule {
    pub field: RuleField,
    pub operator: RuleOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartCollection {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub rules: Vec<SmartRule>,
    pub match_all: bool,
    pub auto_update: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl SmartCollection {
    pub fn new(name: &str, rules: Vec<SmartRule>, match_all: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: String::new(),
            icon: "fa-layer-group".into(),
            rules,
            match_all,
            auto_update: true,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn matches(
        &self,
        tags: &[String],
        rating: u8,
        favorite: bool,
        file_type: &str,
        name: &str,
        notes: &str,
        date_added: Option<DateTime<Utc>>,
        date_modified: Option<DateTime<Utc>>,
        file_size: u64,
    ) -> bool {
        let results: Vec<bool> = self
            .rules
            .iter()
            .map(|rule| {
                let value = rule.value.to_lowercase();
                match &rule.field {
                    RuleField::Tag => tags.iter().any(|t| apply_op(t, &rule.operator, &value)),
                    RuleField::Rating => apply_op(&rating.to_string(), &rule.operator, &value),
                    RuleField::Favorite => apply_op(&favorite.to_string(), &rule.operator, &value),
                    RuleField::Type => apply_op(file_type, &rule.operator, &value),
                    RuleField::Name => apply_op(name, &rule.operator, &value),
                    RuleField::Notes => apply_op(notes, &rule.operator, &value),
                    RuleField::DateAdded => {
                        date_added.map_or(false, |d| apply_date_op(d, &rule.operator, &rule.value))
                    }
                    RuleField::DateModified => {
                        date_modified
                            .map_or(false, |d| apply_date_op(d, &rule.operator, &rule.value))
                    }
                    RuleField::FileSize => {
                        apply_file_size_op(file_size, &rule.operator, &rule.value)
                    }
                }
            })
            .collect();

        if self.match_all {
            results.iter().all(|&r| r)
        } else {
            results.iter().any(|&r| r)
        }
    }
}

fn apply_op(field: &str, op: &RuleOperator, value: &str) -> bool {
    let f = field.to_lowercase();
    match op {
        RuleOperator::Equals => f == value,
        RuleOperator::NotEquals => f != value,
        RuleOperator::GreaterThan => f > value.to_string(),
        RuleOperator::LessThan => f < value.to_string(),
        RuleOperator::Contains => f.contains(value),
        RuleOperator::StartsWith => f.starts_with(value),
        RuleOperator::EndsWith => f.ends_with(value),
        RuleOperator::InRange => false,
    }
}

fn parse_date(value: &str) -> Option<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(d) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        return Some(d.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S") {
        return Some(dt.and_utc());
    }
    None
}

fn apply_date_op(date: DateTime<Utc>, op: &RuleOperator, value: &str) -> bool {
    let parsed = parse_date(value);
    match parsed {
        Some(parsed_date) => match op {
            RuleOperator::Equals => date == parsed_date,
            RuleOperator::NotEquals => date != parsed_date,
            RuleOperator::GreaterThan => date > parsed_date,
            RuleOperator::LessThan => date < parsed_date,
            RuleOperator::InRange => {
                let parts: Vec<&str> = value.split(',').collect();
                if parts.len() == 2 {
                    if let (Some(start), Some(end)) = (parse_date(parts[0]), parse_date(parts[1])) {
                        return date >= start && date <= end;
                    }
                }
                false
            }
            _ => false,
        },
        None => false,
    }
}

fn apply_file_size_op(file_size: u64, op: &RuleOperator, value: &str) -> bool {
    let parsed = value.parse::<u64>();
    match parsed {
        Ok(val) => match op {
            RuleOperator::Equals => file_size == val,
            RuleOperator::NotEquals => file_size != val,
            RuleOperator::GreaterThan => file_size > val,
            RuleOperator::LessThan => file_size < val,
            RuleOperator::InRange => {
                let parts: Vec<&str> = value.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(low), Ok(high)) =
                        (parts[0].parse::<u64>(), parts[1].parse::<u64>())
                    {
                        return file_size >= low && file_size <= high;
                    }
                }
                false
            }
            _ => false,
        },
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_collection_matches_all() {
        let rules = vec![
            SmartRule {
                field: RuleField::Rating,
                operator: RuleOperator::GreaterThan,
                value: "3".into(),
            },
            SmartRule {
                field: RuleField::Favorite,
                operator: RuleOperator::Equals,
                value: "true".into(),
            },
        ];
        let sc = SmartCollection::new("Favorites", rules, true);
        assert!(sc.matches(&[], 4, true, "zip", "test", "", None, None, 0));
        assert!(!sc.matches(&[], 2, true, "zip", "test", "", None, None, 0));
    }

    #[test]
    fn test_smart_collection_matches_any() {
        let rules = vec![
            SmartRule {
                field: RuleField::Tag,
                operator: RuleOperator::Equals,
                value: "logo".into(),
            },
            SmartRule {
                field: RuleField::Tag,
                operator: RuleOperator::Equals,
                value: "branding".into(),
            },
        ];
        let sc = SmartCollection::new("Logo Or Branding", rules, false);
        assert!(sc.matches(&["logo".into()], 0, false, "zip", "", "", None, None, 0));
        assert!(sc.matches(&["branding".into()], 0, false, "zip", "", "", None, None, 0));
        assert!(!sc.matches(&["ui".into()], 0, false, "zip", "", "", None, None, 0));
    }
}
