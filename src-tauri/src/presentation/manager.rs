use super::session::PresentationSession;

pub struct PresentationManager {
    session: Option<PresentationSession>,
}

impl PresentationManager {
    pub fn new() -> Self {
        Self { session: None }
    }

    pub fn start(&mut self, assets: Vec<String>, displays: usize) -> PresentationSession {
        let session = PresentationSession {
            active: true,
            assets,
            current_position: 0,
            displays,
        };
        self.session = Some(session.clone());
        session
    }

    pub fn next(&mut self) {
        if let Some(ref mut session) = self.session {
            session.current_position += session.displays;
        }
    }

    pub fn previous(&mut self) {
        if let Some(ref mut session) = self.session {
            if session.current_position >= session.displays {
                session.current_position -= session.displays;
            }
        }
    }

    pub fn stop(&mut self) {
        self.session = None;
    }
}
