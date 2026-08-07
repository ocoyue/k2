#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instrument {
    symbol: Symbol,

    // 最小价格单位
    tick_size: i64,

    // 最小交易数量单位
    lot_size: u64,
}

impl Instrument {
    pub fn new(symbol: Symbol, tick_size: i64, lot_size: u64) -> Self {
        Self {
            symbol,
            tick_size,
            lot_size,
        }
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn tick_size(&self) -> i64 {
        self.tick_size
    }

    pub fn lot_size(&self) -> u64 {
        self.lot_size
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(String);

impl Symbol {
    pub fn new(v: &str) -> Self {
        Self(v.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
