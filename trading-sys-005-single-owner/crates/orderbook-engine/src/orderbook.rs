use model::Order;

#[derive(Debug, Default)]
pub struct OrderBook {
    orders: Vec<Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn snapshot(&self) -> Vec<Order> {
        self.orders.clone()
    }
}
