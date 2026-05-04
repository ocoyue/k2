use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub struct Summary {
    pub orders_count: u32,
    pub buy_count: u32,
    pub sell_count: u32,
    pub total_value: f64,
}
impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SUMMARY orders_count = {} buy count = {} sell count = {} total_value = {}",
            self.orders_count, self.buy_count, self.sell_count, self.total_value
        )
    }
}
