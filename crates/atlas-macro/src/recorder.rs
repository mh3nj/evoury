use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroStep {
    pub command_id: String,
    pub args: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl MacroStep {
    pub fn new(command_id: &str, args: &str) -> Self {
        Self { command_id: command_id.to_string(), args: args.to_string(), timestamp: chrono::Utc::now() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroRecording {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub steps: Vec<MacroStep>,
    pub shortcut: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl MacroRecording {
    pub fn new(name: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: String::new(),
            steps: Vec::new(),
            shortcut: None,
            created_at: now,
            updated_at: now,
        }
    }
}

pub struct MacroRecorder {
    recordings: Vec<MacroRecording>,
    is_recording: bool,
    current_recording: Option<MacroRecording>,
}

impl MacroRecorder {
    pub fn new() -> Self {
        Self { recordings: Vec::new(), is_recording: false, current_recording: None }
    }

    pub fn start_recording(&mut self, name: &str) -> Result<(), String> {
        if self.is_recording { return Err("Already recording".to_string()); }
        self.is_recording = true;
        self.current_recording = Some(MacroRecording::new(name));
        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<MacroRecording, String> {
        if !self.is_recording { return Err("Not recording".to_string()); }
        self.is_recording = false;
        if let Some(recording) = self.current_recording.take() {
            let rec = recording.clone();
            self.recordings.push(recording);
            return Ok(rec);
        }
        Err("No recording in progress".to_string())
    }

    pub fn record_step(&mut self, command_id: &str, args: &str) -> Result<(), String> {
        if !self.is_recording { return Err("Not recording".to_string()); }
        if let Some(ref mut recording) = self.current_recording {
            recording.steps.push(MacroStep::new(command_id, args));
            recording.updated_at = chrono::Utc::now();
        }
        Ok(())
    }

    pub fn is_recording(&self) -> bool { self.is_recording }
    pub fn current_recording_name(&self) -> Option<String> { self.current_recording.as_ref().map(|r| r.name.clone()) }
    pub fn current_step_count(&self) -> usize { self.current_recording.as_ref().map(|r| r.steps.len()).unwrap_or(0) }

    pub fn get_recording(&self, id: &Uuid) -> Option<&MacroRecording> { self.recordings.iter().find(|r| r.id == *id) }
    pub fn remove_recording(&mut self, id: &Uuid) { self.recordings.retain(|r| r.id != *id); }
    pub fn all_recordings(&self) -> &Vec<MacroRecording> { &self.recordings }
    pub fn rename_recording(&mut self, id: &Uuid, name: &str) {
        if let Some(r) = self.recordings.iter_mut().find(|r| r.id == *id) { r.name = name.to_string(); r.updated_at = chrono::Utc::now(); }
    }
    pub fn count(&self) -> usize { self.recordings.len() }
}
