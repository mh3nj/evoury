use atlas_core::AutoClassifyRule;
use uuid::Uuid;

#[derive(Debug)]
pub struct AutoClassifier {
    rules: Vec<AutoClassifyRule>,
}

impl AutoClassifier {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: AutoClassifyRule) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, id: &Uuid) {
        self.rules.retain(|r| r.id != *id);
    }

    pub fn all_rules(&self) -> Vec<&AutoClassifyRule> {
        self.rules.iter().collect()
    }

    pub fn classify(&self, _name: &str, mime_type: Option<&str>, extension: Option<&str>) -> Vec<&AutoClassifyRule> {
        let mut matched: Vec<&AutoClassifyRule> = self.rules
            .iter()
            .filter(|rule| {
                if let Some(ext) = extension {
                    if rule.matches_extension(ext) {
                        return true;
                    }
                }
                if let Some(mime) = mime_type {
                    if rule.matches_mime(mime) {
                        return true;
                    }
                }
                false
            })
            .collect();
        matched.sort_by(|a, b| b.priority.cmp(&a.priority));
        matched
    }

    pub fn set_rules(&mut self, rules: Vec<AutoClassifyRule>) {
        self.rules = rules;
    }
}
