use event::SequencedEvent;

use journal::JournalFile;

use std::io::{self, ErrorKind};

use std::path::Path;

pub fn replay_journal(
    path: impl AsRef<Path>,
    mut apply: impl FnMut(&SequencedEvent),
) -> io::Result<u64> {
    let events = JournalFile::read_all(path)?;

    let mut expected_seq = 1_u64;

    let mut last_applied_seq = 0_u64;

    for sequenced_event in events {
        if sequenced_event.seq_id() != expected_seq {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "journal sequence gap: \
                         expected {}, got {}",
                    expected_seq,
                    sequenced_event.seq_id(),
                ),
            ));
        }

        apply(&sequenced_event);

        last_applied_seq = sequenced_event.seq_id();

        expected_seq = expected_seq
            .checked_add(1)
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "journal sequence overflow"))?;
    }

    Ok(last_applied_seq)
}
#[cfg(test)]
mod tests {
    use super::replay_journal;

    use event::{EngineEvent, SequencedEvent};

    use journal::JournalFile;

    use std::fs::remove_file;

    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn replay_restores_events_in_sequence() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("replay-{unique}.journal"));

        let mut journal = JournalFile::create_new(&path).unwrap();

        journal
            .append(&SequencedEvent::new(
                1,
                EngineEvent::OrderAdded {
                    id: 1,
                    symbol: "BTCUSDT".to_string(),
                    qty: 10,
                },
            ))
            .unwrap();

        journal
            .append(&SequencedEvent::new(
                2,
                EngineEvent::OrderAdded {
                    id: 2,
                    symbol: "ETHUSDT".to_string(),
                    qty: 20,
                },
            ))
            .unwrap();

        drop(journal);

        let mut ids = Vec::new();

        let last_seq = replay_journal(&path, |seq_event| match seq_event.event() {
            EngineEvent::OrderAdded { id, .. } => {
                ids.push(*id);
            }
        })
        .unwrap();

        assert_eq!(ids, vec![1, 2]);

        assert_eq!(last_seq, 2);

        remove_file(path).unwrap();
    }
}
