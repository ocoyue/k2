use crate::orderbook::OrderBook;
use replay::{replay_journal, replay_journal_from};
use snapshot::{SnapshotData, SnapshotFile};
use std::io;
use std::path::Path;

pub struct RecoveredEngineState {
    pub(crate) book: OrderBook,
    pub(crate) last_applied_seq: u64,
}

pub fn recover_engine_state(
    journal_path: impl AsRef<Path>,
    snapshot_path: impl AsRef<Path>,
) -> io::Result<RecoveredEngineState> {
    let journal_path = journal_path.as_ref();
    let snapshot_path = snapshot_path.as_ref();

    match SnapshotFile::load(snapshot_path) {
        Ok(Some(snapshot)) => match recover_from_snapshot(journal_path, snapshot) {
            Ok(recovered) => {
                println!(
                    "snapshot recovery completed: as_of_seq={}",
                    recovered.last_applied_seq
                );

                return Ok(recovered);
            }

            Err(error) => {
                eprintln!("snapshot recovery failed: {error}; falling back to full replay");
            }
        },

        Ok(None) => {
            println!("snapshot not found; using full journal replay");
        }

        Err(error) => {
            eprintln!("snapshot load failed: {error}; falling back to full replay");
        }
    }

    recover_from_full_journal(journal_path)
}

fn recover_from_snapshot(
    journal_path: &Path,
    snapshot: SnapshotData,
) -> io::Result<RecoveredEngineState> {
    let as_of_seq = snapshot.as_of_seq();
    let journal_offset = snapshot.journal_offset();

    journal::JournalFile::validate_checkpoint(journal_path, journal_offset, as_of_seq)?;

    let mut book = OrderBook::from_orders(snapshot.into_orders());

    let last_applied_seq =
        replay_journal_from(journal_path, journal_offset, as_of_seq, |seq_event| {
            book.apply(seq_event.event());
        })?;

    Ok(RecoveredEngineState {
        book,
        last_applied_seq,
    })
}

fn recover_from_full_journal(journal_path: &Path) -> io::Result<RecoveredEngineState> {
    let mut book = OrderBook::new();

    let last_applied_seq = replay_journal(journal_path, |seq_event| {
        book.apply(seq_event.event());
    })?;

    println!("full replay completed: as_of_seq={last_applied_seq}");

    Ok(RecoveredEngineState {
        book,
        last_applied_seq,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use event::{EngineEvent, SequencedEvent};
    use journal::JournalFile;
    use model::Order;
    use snapshot::{SnapshotData, SnapshotFile};
    use std::fs::{metadata, remove_file};
    use std::time::{SystemTime, UNIX_EPOCH};
    #[test]
    fn recovery_restores_snapshot_then_replays_journal_tail() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let journal_path = std::env::temp_dir().join(format!("snapshot-recovery-{unique}.journal"));

        let snapshot_path =
            std::env::temp_dir().join(format!("snapshot-recovery-{unique}.snapshot"));

        let mut journal = JournalFile::create_new(&journal_path).unwrap();

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

        let journal_offset = metadata(&journal_path).unwrap().len();

        let snapshot = SnapshotData::new(
            2,
            journal_offset,
            vec![
                Order::new(1, "BTCUSDT".to_string(), 10),
                Order::new(2, "ETHUSDT".to_string(), 20),
            ],
        );

        SnapshotFile::save_atomic(&snapshot_path, &snapshot).unwrap();

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

        let recovered = recover_engine_state(&journal_path, &snapshot_path).unwrap();

        assert_eq!(recovered.last_applied_seq, 3);

        let orders = recovered.book.snapshot();

        assert_eq!(orders.len(), 3);
        assert_eq!(orders[2].id(), 3);

        remove_file(journal_path).unwrap();
        remove_file(snapshot_path).unwrap();
    }

    #[test]
    fn broken_snapshot_falls_back_to_full_replay() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let journal_path = std::env::temp_dir().join(format!("fallback-{unique}.journal"));

        let snapshot_path = std::env::temp_dir().join(format!("fallback-{unique}.snapshot"));

        let mut journal = JournalFile::create_new(&journal_path).unwrap();

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

        drop(journal);

        std::fs::write(&snapshot_path, "BROKEN SNAPSHOT").unwrap();

        let recovered = recover_engine_state(&journal_path, &snapshot_path).unwrap();

        assert_eq!(recovered.last_applied_seq, 1);
        assert_eq!(recovered.book.snapshot().len(), 1);

        remove_file(journal_path).unwrap();
        remove_file(snapshot_path).unwrap();
    }
}
