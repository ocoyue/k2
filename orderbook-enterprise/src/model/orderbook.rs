use crate::error::{ExeErr, ParseErr};
use crate::model::{Price, Side, Summary};
use std::collections::{BTreeMap, VecDeque};
use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub struct OrderBook {
    bids: BTreeMap<Price, VecDeque<Order>>,
    asks: BTreeMap<Price, VecDeque<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids:BTreeMap::new(),
            asks:BTreeMap::new(),
        }
    }

    pub fn from_orders(orders: Vec<Order>) -> Result<Self, ExeErr> {
        let mut book = Self::new();

        for order in orders {
            book.add_order(order)?;
        }

        Ok(book)
    }
    pub fn add_order(&mut self, order: Order) -> Result<(), ExeErr> {
        if self.contains_order_id(order.id()) {
            return Err(ExeErr::DuplicateOrderId {
                order_id: order.id(),
            });
        }

        let price = order.price();
        let side = order.side();

        let book_side = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        book_side
            .entry(price)
            .or_insert_with(VecDeque::new)
            .push_back(order);

        Ok(())
    }
    pub fn contains_order_id(&self, order_id: u32) -> bool {
        self.bids
            .values()
            .chain(self.asks.values())
            .flat_map(|level| level.iter())
            .any(|order| order.id() == order_id)
    }
    pub fn best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    pub fn best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    pub fn summary(&self) -> Summary {
        let mut orders_count = 0;
        let mut buy_count = 0;
        let mut sell_count = 0;
        let mut total_value = 0;

        for level in self.bids.values() {
            for order in level {
                orders_count += 1;
                buy_count += 1;
                total_value += order.value();
            }
        }

        for level in self.asks.values() {
            for order in level {
                orders_count += 1;
                sell_count += 1;
                total_value += order.value();
            }
        }

        Summary {
            orders_count,
            buy_count,
            sell_count,
            total_value,
        }
    }
}
impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    id: u32,
    price: Price,
    qty: u32,
    side: Side,
}
impl Order {
    pub fn new(id: u32, price: f64, qty: u32, side: Side) -> Result<Order, ParseErr> {
        if id == 0 {
            return Err(ParseErr::InvalidOrder {
                reason: "id must be positive".to_string(),
            });
        }
        if qty == 0 {
            return Err(ParseErr::InvalidQuantity(qty));
        }
        Ok(Order {
            id,
            price: Price::from_f64(price)?,
            qty,
            side,
        })
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn price(&self) -> Price {
        self.price
    }
    pub fn qty(&self) -> u32 {
        self.qty
    }
    pub fn side(&self) -> Side {
        self.side
    }
    pub fn value(&self) -> i64 {
        self.price.ticks() * (self.qty as i64)
    }

    pub fn set_qty(&mut self, qty: u32) {
        self.qty = qty
    }
}
impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ORDER id={} qty={} price={} side={}",
            self.id, self.qty, self.price, self.side
        )
    }
}
#[test]
fn test_summary_on_bid_ask_book() {
    let mut book = OrderBook::new();

    book.add_order(Order::new(1, 88.00, 100, Side::Buy).unwrap())
        .unwrap();
    book.add_order(Order::new(2, 89.00, 100, Side::Buy).unwrap())
        .unwrap();
    book.add_order(Order::new(3, 90.00, 100, Side::Sell).unwrap())
        .unwrap();

    let summary = book.summary();

    assert_eq!(summary.orders_count, 3);
    assert_eq!(summary.buy_count, 2);
    assert_eq!(summary.sell_count, 1);

    assert_eq!(
        summary.total_value,
        8800 * 100 + 8900 * 100 + 9000 * 100
    );
}