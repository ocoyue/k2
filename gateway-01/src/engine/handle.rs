use std::sync::mpsc::Sender;

use crate::engine::request::EngineRequest;

#[derive(Clone)]
pub struct EngineHandle {
    sender: Sender<EngineRequest>,
}

impl EngineHandle {
    pub fn new(sender: Sender<EngineRequest>) -> Self {
        Self { sender }
    }

    pub fn send(&self, request: EngineRequest) {
        self.sender.send(request).unwrap();
    }
}
