use model::Order;

pub const SNAPSHOT_SCHEMA_VERSION: u64 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotData {
    schema_version: u64,
    as_of_seq: u64,
    journal_offset: u64,
    orders: Vec<Order>,
}

impl SnapshotData {
    pub fn new(as_of_seq: u64, journal_offset: u64, orders: Vec<Order>) -> Self {
        Self {
            schema_version: SNAPSHOT_SCHEMA_VERSION,
            as_of_seq,
            journal_offset,
            orders,
        }
    }

    pub fn schema_version(&self) -> u64 {
        self.schema_version
    }

    pub fn as_of_seq(&self) -> u64 {
        self.as_of_seq
    }

    pub fn journal_offset(&self) -> u64 {
        self.journal_offset
    }

    pub fn orders(&self) -> &[Order] {
        &self.orders
    }

    pub fn into_orders(self) -> Vec<Order> {
        self.orders
    }
}
