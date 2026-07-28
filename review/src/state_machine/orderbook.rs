use crate::model::order::Order;

#[derive(Debug)]
pub struct OrderBook {
    orders: Vec<Order>,
}
impl OrderBook {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }
    pub fn print_all(&self) {
        println!();
        for o in &self.orders {
            println!("{:?}", o);
        }
    }
    pub fn add(&mut self, order: Order) {
        self.orders.push(order);
    }
    pub fn get(&self, id: u32) -> Option<&Order> {
        self.orders.iter().find(|o| o.id() == id)
    }
    pub fn cancel(&mut self, id: u32) {
        self.orders.retain(|o| o.id() != id);
    }

}
