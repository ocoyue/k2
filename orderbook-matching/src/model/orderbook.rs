use crate::error::{ExeErr};
use crate::model::{Price, Side, Summary};
use std::collections::{BTreeMap, HashMap, VecDeque};
use crate::model::order::Order;

pub type OrderId = u32;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderLocation {
    pub side: Side,
    pub price: Price,
}

#[derive(Debug)]
pub struct OrderBook {
    bids: BTreeMap<Price, VecDeque<Order>>,
    asks: BTreeMap<Price, VecDeque<Order>>,
    id_index: HashMap<OrderId, OrderLocation>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids:BTreeMap::new(),
            asks:BTreeMap::new(),
            id_index: HashMap::new(),
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
        let order_id = order.id();
        let price = order.price();
        let side = order.side();

        if self.contains_order_id(order_id) {
            return Err(ExeErr::DuplicateOrderId { order_id });
        }

        let book_side = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        book_side
            .entry(price)
            .or_insert_with(VecDeque::new)
            .push_back(order);

        let old = self.id_index.insert(order_id, OrderLocation { price, side });
        debug_assert!(old.is_none());

        Ok(())
    }
    pub fn contains_order_id(&self, order_id: OrderId) -> bool {
        self.id_index.contains_key(&order_id)
    }
    pub fn best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    pub fn best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    pub fn cancel_order(&mut self, order_id: OrderId) -> Result<(), ExeErr> {
        let location = self
            .id_index
            .get(&order_id)
            .copied()
            .ok_or(ExeErr::OrderNotFound { order_id })?;

        let book_side = match location.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        let level = book_side
            .get_mut(&location.price)
            .ok_or(ExeErr::OrderNotFound { order_id })?;

        let pos = level
            .iter()
            .position(|order| order.id() == order_id)
            .ok_or(ExeErr::OrderNotFound { order_id })?;

        let removed = level.remove(pos);
        debug_assert!(removed.is_some());
        if level.is_empty() {
            book_side.remove(&location.price);
        }

        self.id_index.remove(&order_id);

        Ok(())
    }
    fn find_order_ref(&self, order_id: OrderId) -> Option<&Order> {
        let location = self.id_index.get(&order_id).copied()?;
        let level = self.level_ref(location)?;

        level.iter().find(|order| order.id() == order_id)
    }
    fn level_ref(&self, location: OrderLocation) -> Option<&VecDeque<Order>> {
        match location.side {
            Side::Buy => self.bids.get(&location.price),
            Side::Sell => self.asks.get(&location.price),
        }
    }

    pub fn get_order(&self, order_id: OrderId) -> Result<Order, ExeErr> {
        self.find_order_ref(order_id)
            .cloned()
            .ok_or(ExeErr::OrderNotFound { order_id })
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
    pub fn check_invariants(&self) -> Result<(), String> {
        for (price, level) in &self.bids {
            if level.is_empty() {
                return Err(format!("empty bid price level: {}", price));
            }

            for order in level {
                let order_id = order.id();

                let location = self
                    .id_index
                    .get(&order_id)
                    .ok_or_else(|| format!("missing id_index for bid order_id={}", order_id))?;

                if location.side != Side::Buy {
                    return Err(format!(
                        "side mismatch for order_id={}: index={:?}, book=Buy",
                        order_id, location.side
                    ));
                }

                if location.price != *price {
                    return Err(format!(
                        "price mismatch for order_id={}: index={}, book={}",
                        order_id, location.price, price
                    ));
                }
            }
        }

        for (price, level) in &self.asks {
            if level.is_empty() {
                return Err(format!("empty ask price level: {}", price));
            }

            for order in level {
                let order_id = order.id();

                let location = self
                    .id_index
                    .get(&order_id)
                    .ok_or_else(|| format!("missing id_index for ask order_id={}", order_id))?;

                if location.side != Side::Sell {
                    return Err(format!(
                        "side mismatch for order_id={}: index={:?}, book=Sell",
                        order_id, location.side
                    ));
                }

                if location.price != *price {
                    return Err(format!(
                        "price mismatch for order_id={}: index={}, book={}",
                        order_id, location.price, price
                    ));
                }
            }
        }

        for order_id in self.id_index.keys() {
            if self.find_order_ref(*order_id).is_none() {
                return Err(format!("id_index points to missing order_id={}", order_id));
            }
        }

        Ok(())
    }
}
impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

