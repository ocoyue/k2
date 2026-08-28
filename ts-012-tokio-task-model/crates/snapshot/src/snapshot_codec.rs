use crate::snapshot_data::{SNAPSHOT_SCHEMA_VERSION, SnapshotData};
use model::Order;
use std::io::{self, ErrorKind};
pub(crate) fn encode_snapshot(snapshot: &SnapshotData) -> String {
    let mut output = format!(
        "SNAPSHOT|{}|{}|{}|{}\n",
        snapshot.schema_version(),
        snapshot.as_of_seq(),
        snapshot.journal_offset(),
        snapshot.orders().len(),
    );

    for order in snapshot.orders() {
        output.push_str(&format!(
            "ORDER|{}|{}|{}\n",
            order.id(),
            order.symbol(),
            order.qty(),
        ));
    }

    output
}

pub(crate) fn decode_snapshot(input: &str) -> io::Result<SnapshotData> {
    let mut lines = input.lines();

    let header = lines
        .next()
        .ok_or_else(|| invalid_data("snapshot is empty"))?;
    let header_parts: Vec<&str> = header.split('|').collect();

    if header_parts.len() != 5 || header_parts[0] != "SNAPSHOT" {
        return Err(invalid_data("invalid snapshot header"));
    }

    let schema_version = parse_u64(header_parts[1], "schema version")?;

    if schema_version != SNAPSHOT_SCHEMA_VERSION {
        return Err(invalid_data(format!(
            "unsupported snapshot schema version: {schema_version}"
        )));
    }

    let as_of_seq = parse_u64(header_parts[2], "as_of_seq")?;
    let journal_offset = parse_u64(header_parts[3], "journal_offset")?;
    let expected_order_count = parse_u64(header_parts[4], "order count")? as usize;

    let mut orders = Vec::with_capacity(expected_order_count);

    for line in lines {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() != 4 || parts[0] != "ORDER" {
            return Err(invalid_data("invalid snapshot order record"));
        }

        let id = parse_u64(parts[1], "order id")?;
        let symbol = parts[2].to_string();
        let qty = parse_u64(parts[3], "order qty")?;

        orders.push(Order::new(id, symbol, qty));
    }

    if orders.len() != expected_order_count {
        return Err(invalid_data(format!(
            "snapshot order count mismatch: expected {}, got {}",
            expected_order_count,
            orders.len(),
        )));
    }

    Ok(SnapshotData::new(as_of_seq, journal_offset, orders))
}

fn parse_u64(value: &str, field: &str) -> io::Result<u64> {
    value
        .parse::<u64>()
        .map_err(|_| invalid_data(format!("invalid snapshot {field}")))
}

fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, message.into())
}
#[cfg(test)]
mod tests {
    use super::*;
    use model::Order;

    #[test]
    fn snapshot_codec_round_trip() {
        let snapshot = SnapshotData::new(
            3,
            87,
            vec![
                Order::new(1, "BTCUSDT".to_string(), 10),
                Order::new(2, "ETHUSDT".to_string(), 20),
            ],
        );

        let encoded = encode_snapshot(&snapshot);
        let decoded = decode_snapshot(&encoded).unwrap();

        assert_eq!(decoded, snapshot);
    }

    #[test]
    fn decode_rejects_wrong_order_count() {
        let input = concat!("SNAPSHOT|1|3|87|2\n", "ORDER|1|BTCUSDT|10\n",);

        assert!(decode_snapshot(input).is_err());
    }
}
