use event::EngineEvent;
use model::Order;

#[derive(Debug, Default)]
pub struct OrderBook {
    orders: Vec<Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, event: &EngineEvent) {
        match event {
            EngineEvent::OrderAdded { id, symbol, qty } => {
                let order = Order::new(*id, symbol.clone(), *qty);

                self.orders.push(order);
            }
        }
    }

    pub fn snapshot(&self) -> Vec<Order> {
        self.orders.clone()
    }
    pub fn from_orders(orders: Vec<Order>) -> Self {
        Self { orders }
    }
}
