#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequencedEvent {
    seq_id: u64,
    event: EngineEvent,
}

impl SequencedEvent {
    pub fn new(seq_id: u64, event: EngineEvent) -> Self {
        Self { seq_id, event }
    }

    pub fn seq_id(&self) -> u64 {
        self.seq_id
    }

    pub fn event(&self) -> &EngineEvent {
        &self.event
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineEvent {
    OrderAdded { id: u64, symbol: String, qty: u64 },
}
