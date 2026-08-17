use event::{EngineEvent, SequencedEvent};

pub(crate) fn encode_record(sequenced_event: &SequencedEvent) -> String {
    match sequenced_event.event() {
        EngineEvent::OrderAdded { id, symbol, qty } => {
            format!(
                "{}|ORDER_ADDED|{}|{}|{}\n",
                sequenced_event.seq_id(),
                id,
                symbol,
                qty,
            )
        }
    }
}
