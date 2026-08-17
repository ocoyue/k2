use crate::book::MiniOrderBook;
use model::Order;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct BookService {
    book: Arc<Mutex<MiniOrderBook>>,
}

impl BookService {
    pub fn new(book: Arc<Mutex<MiniOrderBook>>) -> Self {
        Self { book }
    }

    pub fn add_order(&self, id: u64, symbol: String, qty: u64) -> AddOrderResult {
        let order = Order::new(id, symbol, qty);

        let mut guard_book = self.book.lock().expect("orderbook mutex poisoned");

        guard_book.add(order);

        AddOrderResult { id }
    }

    pub fn get_book(&self) -> BookSnapshot {
        let guard_book = self.book.lock().expect("orderbook mutex poisoned");
        BookSnapshot {
            orders: guard_book.snapshot(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AddOrderResult {
    pub id: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BookSnapshot {
    pub orders: Vec<Order>,
}
#[cfg(test)]
mod tests {
    use super::BookService;
    use crate::MiniOrderBook;

    use std::sync::{Arc, Mutex};

    #[test]
    fn two_services_share_same_book() {
        let book = Arc::new(Mutex::new(MiniOrderBook::new()));

        let service_a = BookService::new(Arc::clone(&book));

        let service_b = BookService::new(Arc::clone(&book));

        service_a.add_order(1, "BTCUSDT".to_string(), 10);

        let snapshot = service_b.get_book();

        assert_eq!(snapshot.orders.len(), 1);

        assert_eq!(snapshot.orders[0].id(), 1);
    }
}
