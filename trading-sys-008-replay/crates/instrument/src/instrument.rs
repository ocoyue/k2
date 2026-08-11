#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrumentStatus {
    Active,
    Halted,
}

impl InstrumentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InstrumentStatus::Active => "ACTIVE",
            InstrumentStatus::Halted => "HALTED",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instrument {
    symbol: String,
    tick_size: u64,
    lot_size: u64,
    status: InstrumentStatus,
}

impl Instrument {
    pub fn new(
        symbol: impl Into<String>,
        tick_size: u64,
        lot_size: u64,
        status: InstrumentStatus,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            tick_size,
            lot_size,
            status,
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn tick_size(&self) -> u64 {
        self.tick_size
    }

    pub fn lot_size(&self) -> u64 {
        self.lot_size
    }

    pub fn status(&self) -> InstrumentStatus {
        self.status
    }
}
