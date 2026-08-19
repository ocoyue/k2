use crate::orderbook::OrderBook;
use snapshot::{SnapshotData, SnapshotFile};
use std::io;
use std::path::Path;

pub(crate) fn create_checkpoint(
    snapshot_path: impl AsRef<Path>,
    book: &OrderBook,
    as_of_seq: u64,
    journal_offset: u64,
) -> io::Result<()> {
    let snapshot = SnapshotData::new(as_of_seq, journal_offset, book.snapshot());

    SnapshotFile::save_atomic(snapshot_path, &snapshot)
}
#[cfg(test)]
mod tests {
    use super::*;
    use model::Order;
    use snapshot::SnapshotFile;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn checkpoint_captures_book_state_and_journal_position() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("checkpoint-{unique}.snapshot"));

        let book = OrderBook::from_orders(vec![
            Order::new(1, "BTCUSDT".to_string(), 10),
            Order::new(2, "ETHUSDT".to_string(), 20),
        ]);

        create_checkpoint(&path, &book, 2, 128).unwrap();

        let snapshot = SnapshotFile::load(&path).unwrap().unwrap();

        assert_eq!(snapshot.as_of_seq(), 2);
        assert_eq!(snapshot.journal_offset(), 128);
        assert_eq!(snapshot.orders().len(), 2);

        std::fs::remove_file(path).unwrap();
    }
}
