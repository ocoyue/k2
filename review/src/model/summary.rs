#[derive(Debug, PartialEq, Eq)]
pub struct OrderBookSummary {
    pub order_count: usize,
    pub buy_count: usize,
    pub sell_count: usize,
    pub total_quantity: u64,
}
