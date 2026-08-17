use crate::journal_codec::{decode_record, encode_record};
use event::SequencedEvent;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::path::Path;

#[derive(Debug)]
pub struct JournalFile {
    file: File,
}

impl JournalFile {
    pub fn create_new(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                create_dir_all(parent)?;
            }
        }

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create_new(true)
            .open(path)?;

        Ok(Self { file })
    }
    pub fn open_or_create(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                create_dir_all(parent)?;
            }
        }

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;

        Ok(Self { file })
    }

    pub fn append(&mut self, event: &SequencedEvent) -> io::Result<()> {
        let record = encode_record(event);

        self.file.write_all(record.as_bytes())?;

        self.file.sync_data()?;

        Ok(())
    }

    pub fn read_all(path: impl AsRef<Path>) -> io::Result<Vec<SequencedEvent>> {
        let path = path.as_ref();

        let file = match File::open(path) {
            Ok(file) => file,

            Err(error) if error.kind() == ErrorKind::NotFound => {
                // 代表没有历史journal，从零建立
                return Ok(Vec::new());
            }

            Err(error) => {
                return Err(error);
            }
        };

        let reader = BufReader::new(file);

        let mut events = Vec::new();

        for (line_index, line_result) in reader.lines().enumerate() {
            let line = line_result?;

            let event = decode_record(&line).map_err(|error| {
                io::Error::new(
                    ErrorKind::InvalidData,
                    format!("journal line {}: {}", line_index + 1, error,),
                )
            })?;

            events.push(event);
        }

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::JournalFile;

    use event::{EngineEvent, SequencedEvent};

    use std::fs::{read_to_string, remove_file};

    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn append_writes_events_in_order() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("trading-journal-{unique}.log"));

        let mut journal = JournalFile::create_new(&path).unwrap();

        let first = SequencedEvent::new(
            1,
            EngineEvent::OrderAdded {
                id: 1,
                symbol: "BTCUSDT".to_string(),
                qty: 10,
            },
        );

        let second = SequencedEvent::new(
            2,
            EngineEvent::OrderAdded {
                id: 2,
                symbol: "ETHUSDT".to_string(),
                qty: 20,
            },
        );

        journal.append(&first).unwrap();

        journal.append(&second).unwrap();

        drop(journal);

        let content = read_to_string(&path).unwrap();

        assert_eq!(
            content,
            concat!(
                "1|ORDER_ADDED|1|BTCUSDT|10\n",
                "2|ORDER_ADDED|2|ETHUSDT|20\n",
            )
        );

        remove_file(path).unwrap();
    }
}
