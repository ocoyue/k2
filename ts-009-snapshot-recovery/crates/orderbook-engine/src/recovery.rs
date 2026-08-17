use crate::orderbook::OrderBook;
use replay::replay_journal;
use std::io;
use std::path::Path;

pub struct RecoveredEngineState {
    pub(crate) book: OrderBook,
    pub(crate) last_applied_seq: u64,
}

pub fn recover_engine_state(journal_path: impl AsRef<Path>) -> io::Result<RecoveredEngineState> {
    let mut book = OrderBook::new();

    let last_applied_seq = replay_journal(journal_path, |seq_event| {
        book.apply(seq_event.event());
    })?;

    println!("recovery completed: as_of_seq={last_applied_seq}");

    Ok(RecoveredEngineState {
        book,
        last_applied_seq,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use event::{EngineEvent, SequencedEvent};
    use journal::JournalFile;
    use std::fs::remove_file;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn recovery_restores_old_state() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("recovery-{unique}.journal"));

        {
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
        }

        let recovered = recover_engine_state(&path).unwrap();

        assert_eq!(recovered.last_applied_seq, 1);
        assert_eq!(recovered.book.snapshot().len(), 1);

        remove_file(path).unwrap();
    }
}
