use event::{EngineEvent, SequencedEvent};

use std::io::{self, ErrorKind};
const JOURNAL_LENGTH: usize = 5;

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

pub(crate) fn decode_record(record: &str) -> io::Result<SequencedEvent> {
    let parts: Vec<&str> = record.split('|').collect();

    if parts.len() != JOURNAL_LENGTH {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("invalid journal record field count: {}", parts.len()),
        ));
    }

    let seq_id = parts[0]
        .parse::<u64>()
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "invalid journal seq_id"))?;

    let event = match parts[1] {
        "ORDER_ADDED" => {
            let id = parts[2]
                .parse::<u64>()
                .map_err(|_| io::Error::new(ErrorKind::InvalidData, "invalid order id"))?;

            let symbol = parts[3].to_string();

            let qty = parts[4]
                .parse::<u64>()
                .map_err(|_| io::Error::new(ErrorKind::InvalidData, "invalid order qty"))?;

            EngineEvent::OrderAdded { id, symbol, qty }
        }

        event_type => {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("unknown journal event type: {event_type}"),
            ));
        }
    };

    Ok(SequencedEvent::new(seq_id, event))
}

#[test]
fn decode_record_restores_event() {
    let event = decode_record("7|ORDER_ADDED|100|BTCUSDT|10").unwrap();

    assert_eq!(event.seq_id(), 7);

    assert_eq!(
        event.event(),
        &EngineEvent::OrderAdded {
            id: 100,
            symbol: "BTCUSDT".to_string(),
            qty: 10,
        }
    );
}
