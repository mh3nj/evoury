use crossbeam_channel::{Receiver, Sender, unbounded};
use crate::AtlasEvent;

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
