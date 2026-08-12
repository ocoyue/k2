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
