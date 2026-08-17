use crate::book::MiniOrderBook;
use model::Order;

#[derive(Debug)]
pub struct BookService {
    book: MiniOrderBook,
}

impl BookService {
    pub fn new(book: MiniOrderBook) -> Self {
        Self { book }
    }

    pub fn add_order(&mut self, id: u64, symbol: String, qty: u64) -> AddOrderResult {
        let order = Order::new(id, symbol, qty);

        self.book.add(order);

        AddOrderResult { id }
    }

    pub fn get_book(&self) -> BookSnapshot {
        BookSnapshot {
            orders: self.book.snapshot(),
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
