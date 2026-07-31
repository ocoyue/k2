use crate::error::order_error::OrderError;
use crate::model::side::Side;

#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    id: u32,
    side: Side,
    price: f64,
    quantity: u64,
}
impl Order {
    pub fn new(id: u32, side: Side, price: f64, quantity: u64) -> Result<Self, OrderError> {
        if price <= 0.0 {
            return Err(OrderError::NegativePrice);
        }
        if quantity <= 0 {
            return Err(OrderError::ZeroQuantity);
        }

        Ok(Self {
            id,
            side,
            price,
            quantity,
        })
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn quantity(&self) -> u64 {
        self.quantity
    }
    pub fn is_buy(&self) -> bool {
        self.side.is_buy()
    }
    pub fn is_sell(&self) -> bool {
        self.side.is_sell()
    }
    pub fn reduce(&mut self, amount: u64) -> Result<u64, OrderError> {
        if amount > self.quantity {
            Err(OrderError::ReduceAmountExceedsRemaining)
        } else {
            self.quantity -= amount;
            Ok(self.quantity)
        }
    }
    pub fn is_filled(&self) -> bool {
        self.quantity == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_should_create_order() {
        let order_res = Order::new(1, Side::Buy, 100.25, 10);

        matches!(order_res, Ok(_));
        let order = order_res.unwrap();
        assert_eq!(order.id, 1);
        assert_eq!(order.side, Side::Buy);
        assert_eq!(order.price, 100.25);
        assert_eq!(order.quantity, 10);
    }
    #[test]
    fn id_should_return_borrowed_order_id() {
        let order = Order::new(1, Side::Buy, 100.25, 10).unwrap();

        assert_eq!(order.id(), 1);
        assert_eq!(order.quantity, 10);
    }

    #[test]
    fn buy_order_should_be_buy() {
        let order = Order::new(1, Side::Buy, 100.25, 10).unwrap();

        assert!(order.is_buy());
        assert!(!order.is_sell());
    }

    #[test]
    fn reduce_should_decrease_quantity() {
        let mut order = Order::new(1, Side::Buy, 100.25, 10).unwrap();

        order.reduce(3);

        assert_eq!(order.quantity, 7);
    }
    #[test]
    fn reduce_should_not_underflow() {
        let mut order = Order::new(1, Side::Buy, 100.25, 10).unwrap();
        let result = order.reduce(20);
        assert!(result.is_err());
        assert_eq!(order.quantity(), 10);
    }
    #[test]
    fn zero_quantity_order_should_be_filled() {
        let mut order = Order::new(1, Side::Sell, 101.50, 5).unwrap();

        order.reduce(5);

        assert!(order.is_filled());
    }
}
