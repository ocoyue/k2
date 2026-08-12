use crate::{EngineEvent, SequencedEvent};

#[derive(Debug)]
pub struct Sequencer {
    next_seq: u64,
}

impl Sequencer {
    pub fn new() -> Self {
        Self { next_seq: 1 }
    }

    pub fn assign_sequence(&mut self, event: EngineEvent) -> SequencedEvent {
        let seq_id = self.next_seq;

        self.next_seq = self
            .next_seq
            .checked_add(1)
            .expect("event sequence overflow");

        SequencedEvent::new(seq_id, event)
    }
    pub fn resume_after(last_seq: u64) -> Self {
        let next_seq = last_seq.checked_add(1).expect("event sequence overflow");

        Self { next_seq }
    }
}

impl Default for Sequencer {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::EngineEvent;

    fn event(id: u64) -> EngineEvent {
        EngineEvent::OrderAdded {
            id,
            symbol: "BTCUSDT".to_string(),
            qty: 10,
        }
    }

    #[test]
    fn sequence_starts_at_one() {
        let mut sequencer = Sequencer::new();

        let first = sequencer.assign_sequence(event(1));

        assert_eq!(first.seq_id(), 1);
    }

    #[test]
    fn sequence_is_monotonic() {
        let mut sequencer = Sequencer::new();

        let first = sequencer.assign_sequence(event(1));

        let second = sequencer.assign_sequence(event(2));

        assert_eq!(first.seq_id(), 1);

        assert_eq!(second.seq_id(), 2);
    }
}
