use crate::journal_codec::{decode_record, encode_record};
use event::SequencedEvent;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom, Write};
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
        Self::read_from(path, 0)
    }

    pub fn read_from(path: impl AsRef<Path>, offset: u64) -> io::Result<Vec<SequencedEvent>> {
        let path = path.as_ref();

        let mut file = match File::open(path) {
            Ok(file) => file,

            Err(error) if error.kind() == ErrorKind::NotFound && offset == 0 => {
                return Ok(Vec::new());
            }

            Err(error) => return Err(error),
        };

        let file_len = file.metadata()?.len();

        if offset > file_len {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("journal offset {offset} exceeds file length {file_len}"),
            ));
        }

        validate_record_boundary(&mut file, offset)?;

        file.seek(SeekFrom::Start(offset))?;

        let reader = BufReader::new(file);
        let mut events = Vec::new();

        for (line_index, line_result) in reader.lines().enumerate() {
            let line = line_result?;

            let event = decode_record(&line).map_err(|error| {
                io::Error::new(
                    ErrorKind::InvalidData,
                    format!("journal tail line {}: {error}", line_index + 1),
                )
            })?;

            events.push(event);
        }

        Ok(events)
    }
    pub fn validate_checkpoint(
        path: impl AsRef<Path>,
        offset: u64,
        expected_seq: u64,
    ) -> io::Result<()> {
        if offset == 0 {
            if expected_seq == 0 {
                return Ok(());
            }

            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("journal checkpoint mismatch: offset=0 but expected_seq={expected_seq}"),
            ));
        }

        let path = path.as_ref();
        let mut file = File::open(path)?;

        let file_len = file.metadata()?.len();

        if offset > file_len {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("journal offset {offset} exceeds file length {file_len}"),
            ));
        }

        validate_record_boundary(&mut file, offset)?;

        let record_end = offset - 1;
        let mut record_start = record_end;

        while record_start > 0 {
            file.seek(SeekFrom::Start(record_start - 1))?;

            let mut byte = [0_u8; 1];
            file.read_exact(&mut byte)?;

            if byte[0] == b'\n' {
                break;
            }

            record_start -= 1;
        }

        let record_len = (record_end - record_start) as usize;
        let mut record_bytes = vec![0_u8; record_len];

        file.seek(SeekFrom::Start(record_start))?;
        file.read_exact(&mut record_bytes)?;

        let record = std::str::from_utf8(&record_bytes)
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "invalid journal utf-8"))?;

        let seq_event = decode_record(record).map_err(|error| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("invalid journal checkpoint record: {error}"),
            )
        })?;

        if seq_event.seq_id() != expected_seq {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "journal checkpoint mismatch: expected seq {}, got {}",
                    expected_seq,
                    seq_event.seq_id(),
                ),
            ));
        }

        Ok(())
    }
    // pub fn read_all2(path: impl AsRef<Path>) -> io::Result<Vec<SequencedEvent>> {
    //     let path = path.as_ref();
    //
    //     let file = match File::open(path) {
    //         Ok(file) => file,
    //
    //         Err(error) if error.kind() == ErrorKind::NotFound => {
    //             // 代表没有历史journal，从零建立
    //             return Ok(Vec::new());
    //         }
    //
    //         Err(error) => {
    //             return Err(error);
    //         }
    //     };
    //
    //     let reader = BufReader::new(file);
    //
    //     let mut events = Vec::new();
    //
    //     for (line_index, line_result) in reader.lines().enumerate() {
    //         let line = line_result?;
    //
    //         let event = decode_record(&line).map_err(|error| {
    //             io::Error::new(
    //                 ErrorKind::InvalidData,
    //                 format!("journal line {}: {}", line_index + 1, error,),
    //             )
    //         })?;
    //
    //         events.push(event);
    //     }
    //
    //     Ok(events)
    // }
}
fn validate_record_boundary(file: &mut File, offset: u64) -> io::Result<()> {
    if offset == 0 {
        return Ok(());
    }

    file.seek(SeekFrom::Start(offset - 1))?;

    let mut byte = [0_u8; 1];
    file.read_exact(&mut byte)?;

    if byte[0] != b'\n' {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("journal offset {offset} is not a record boundary"),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::JournalFile;
    use event::{EngineEvent, SequencedEvent};
    use std::fs::{metadata, read_to_string, remove_file};
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
    #[test]
    fn read_from_reads_only_journal_tail() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("journal-tail-{unique}.log"));

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

        let events = JournalFile::read_from(&path, offset).unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].seq_id(), 3);

        remove_file(path).unwrap();
    }
}
