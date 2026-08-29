use crate::SnapshotData;
use crate::snapshot_codec::{decode_snapshot, encode_snapshot};
use std::ffi::OsString;
use std::fs::{File, OpenOptions, create_dir_all, rename};
use std::io::{self, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

pub struct SnapshotFile;

impl SnapshotFile {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Option<SnapshotData>> {
        let path = path.as_ref();

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error),
        };

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(Some(decode_snapshot(&content)?))
    }

    pub fn save_atomic(path: impl AsRef<Path>, snapshot: &SnapshotData) -> io::Result<()> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                create_dir_all(parent)?;
            }
        }

        let temp_path = temporary_path(path);
        let content = encode_snapshot(snapshot);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)?;

        file.write_all(content.as_bytes())?;
        file.sync_data()?;
        drop(file);

        rename(&temp_path, path)?;

        Ok(())
    }
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut temp_name: OsString = path.as_os_str().to_os_string();
    temp_name.push(".tmp");

    PathBuf::from(temp_name)
}
#[cfg(test)]
mod tests {
    use super::*;
    use model::Order;
    use std::fs::remove_file;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("{name}-{unique}.snapshot"))
    }

    #[test]
    fn save_and_load_snapshot() {
        let path = unique_path("snapshot-round-trip");

        let snapshot = SnapshotData::new(2, 64, vec![Order::new(1, "BTCUSDT".to_string(), 10)]);

        SnapshotFile::save_atomic(&path, &snapshot).unwrap();

        let loaded = SnapshotFile::load(&path).unwrap().unwrap();

        assert_eq!(loaded, snapshot);

        remove_file(path).unwrap();
    }

    #[test]
    fn save_atomic_replaces_old_snapshot() {
        let path = unique_path("snapshot-replace");

        let first = SnapshotData::new(1, 32, vec![Order::new(1, "BTCUSDT".to_string(), 10)]);

        SnapshotFile::save_atomic(&path, &first).unwrap();

        let second = SnapshotData::new(
            2,
            64,
            vec![
                Order::new(1, "BTCUSDT".to_string(), 10),
                Order::new(2, "ETHUSDT".to_string(), 20),
            ],
        );

        SnapshotFile::save_atomic(&path, &second).unwrap();

        let loaded = SnapshotFile::load(&path).unwrap().unwrap();

        assert_eq!(loaded, second);

        remove_file(path).unwrap();
    }

    #[test]
    fn load_missing_snapshot_returns_none() {
        let path = unique_path("missing-snapshot");

        let loaded = SnapshotFile::load(path).unwrap();

        assert!(loaded.is_none());
    }
}
