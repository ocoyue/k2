#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: u64,
    symbol: String,
    qty: u64,
}

impl Order {
    pub fn new(id: u64, symbol: String, qty: u64) -> Self {
        Self { id, symbol, qty }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn qty(&self) -> u64 {
        self.qty
    }
}
