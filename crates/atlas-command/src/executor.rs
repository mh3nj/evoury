use std::collections::HashMap;
use crate::{Command, CommandResult};

pub type CommandHandler = Box<dyn Fn(&Command) -> CommandResult + Send + Sync>;

pub struct CommandExecutor {
    handlers: HashMap<String, CommandHandler>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, command_id: &str, handler: F)
    where
        F: Fn(&Command) -> CommandResult + Send + Sync + 'static,
    {
        self.handlers.insert(command_id.to_string(), Box::new(handler));
    }

    pub fn execute(&self, command: &Command) -> Option<CommandResult> {
        self.handlers
            .get(&command.id.to_string())
            .map(|handler| handler(command))
    }

    pub fn has_handler(&self, command_id: &str) -> bool {
        self.handlers.contains_key(command_id)
    }

    pub fn registered_ids(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }
}
