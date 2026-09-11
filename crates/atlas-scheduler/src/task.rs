#[derive(Debug, Clone)]
pub enum TaskPriority {
    Critical,
    High,
    Normal,
    Low,
    IdleOnly,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub priority: TaskPriority,
}
