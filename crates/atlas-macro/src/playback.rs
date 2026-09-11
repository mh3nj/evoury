use crate::recorder::{MacroRecording, MacroStep};

pub struct MacroPlayback {
    is_playing: bool,
    current_index: usize,
    current_macro: Option<MacroRecording>,
}

impl MacroPlayback {
    pub fn new() -> Self { Self { is_playing: false, current_index: 0, current_macro: None } }

    pub fn load(&mut self, recording: MacroRecording) { self.current_macro = Some(recording); self.current_index = 0; }

    pub fn start(&mut self) -> Result<(), String> {
        if self.current_macro.is_none() { return Err("No macro loaded".to_string()); }
        self.is_playing = true;
        self.current_index = 0;
        Ok(())
    }

    pub fn stop(&mut self) { self.is_playing = false; self.current_index = 0; }
    pub fn pause(&mut self) { self.is_playing = false; }

    pub fn next_step(&mut self) -> Option<&MacroStep> {
        if !self.is_playing { return None; }
        if let Some(ref recording) = self.current_macro {
            if self.current_index < recording.steps.len() {
                let step = &recording.steps[self.current_index];
                self.current_index += 1;
                return Some(step);
            }
            self.is_playing = false;
        }
        None
    }

    pub fn is_playing(&self) -> bool { self.is_playing }
    pub fn progress(&self) -> (usize, usize) {
        match &self.current_macro {
            Some(m) => (self.current_index, m.steps.len()),
            None => (0, 0),
        }
    }
    pub fn loaded_macro_name(&self) -> Option<String> { self.current_macro.as_ref().map(|m| m.name.clone()) }
}
