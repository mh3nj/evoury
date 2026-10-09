use crate::AtlasEvent;
use crossbeam_channel::{unbounded, Receiver, Sender};

pub struct EventBus {
    sender: Sender<AtlasEvent>,
    receiver: Receiver<AtlasEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }

    pub fn sender(&self) -> Sender<AtlasEvent> {
        self.sender.clone()
    }

    pub fn subscribe(&self) -> Receiver<AtlasEvent> {
        self.receiver.clone()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
