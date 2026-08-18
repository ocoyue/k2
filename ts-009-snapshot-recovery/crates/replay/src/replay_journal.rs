use event::SequencedEvent;
use journal::JournalFile;
use std::io::{self, ErrorKind};
use std::path::Path;

pub fn replay_journal(
    path: impl AsRef<Path>,
    apply: impl FnMut(&SequencedEvent),
) -> io::Result<u64> {
    replay_journal_from(path, 0, 0, apply)
}

pub fn replay_journal_from(
    path: impl AsRef<Path>,
    journal_offset: u64,
    last_applied_seq: u64,
    mut apply: impl FnMut(&SequencedEvent),
) -> io::Result<u64> {
    let events = JournalFile::read_from(path, journal_offset)?;

    let mut expected_seq = last_applied_seq
        .checked_add(1)
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "journal sequence overflow"))?;

    let mut current_seq = last_applied_seq;

    for seq_event in events {
        if seq_event.seq_id() != expected_seq {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "journal sequence gap: expected {}, got {}",
                    expected_seq,
                    seq_event.seq_id(),
                ),
            ));
        }

        apply(&seq_event);

        current_seq = seq_event.seq_id();

        expected_seq = expected_seq
            .checked_add(1)
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "journal sequence overflow"))?;
    }

    Ok(current_seq)
}
#[cfg(test)]
mod tests {
    use super::*;
    use event::EngineEvent;
    use std::fs::{metadata, remove_file};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn tail_replay_continues_after_snapshot_sequence() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("tail-replay-{unique}.journal"));

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

        let offset = metadata(&path).unwrap().len();

        journal
            .append(&SequencedEvent::new(
                3,
                EngineEvent::OrderAdded {
                    id: 3,
                    symbol: "SOLUSDT".to_string(),
                    qty: 30,
                },
            ))
            .unwrap();

        drop(journal);

        let mut ids = Vec::new();

        let last_seq = replay_journal_from(&path, offset, 2, |seq_event| match seq_event.event() {
            EngineEvent::OrderAdded { id, .. } => ids.push(*id),
        })
        .unwrap();

        assert_eq!(ids, vec![3]);
        assert_eq!(last_seq, 3);

        remove_file(path).unwrap();
    }
}
