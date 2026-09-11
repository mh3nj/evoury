use std::collections::HashMap;
use crate::Command;

pub struct CommandRegistry {
    commands: HashMap<String, Command>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, command: Command) {
        self.commands.insert(command.id.to_string(), command);
    }

    pub fn all(&self) -> Vec<Command> {
        let mut cmds: Vec<Command> = self.commands.values().cloned().collect();
        cmds.sort_by(|a, b| a.name.cmp(&b.name));
        cmds
    }

    pub fn find(&self, id: &str) -> Option<&Command> {
        self.commands.get(id)
    }

    pub fn search(&self, query: &str) -> Vec<Command> {
        let q = query.to_lowercase();
        self.commands
            .values()
            .filter(|c| {
                c.name.to_lowercase().contains(&q)
                    || c.description.to_lowercase().contains(&q)
                    || c.keywords.iter().any(|k| k.to_lowercase().contains(&q))
            })
            .cloned()
            .collect()
    }

    pub fn by_category(&self, category: &str) -> Vec<Command> {
        self.commands
            .values()
            .filter(|c| format!("{:?}", c.category).to_lowercase() == category.to_lowercase())
            .cloned()
            .collect()
    }

    pub fn count(&self) -> usize {
        self.commands.len()
    }
}
