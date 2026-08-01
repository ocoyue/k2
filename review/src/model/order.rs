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
        if !price.is_finite() || price <= 0.0 {
            return Err(OrderError::InvalidPrice);
        }
        if quantity <= 0 {
            return Err(OrderError::InvalidQty);
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
        if amount == 0 {
            return Err(OrderError::ZeroReduceAmount);
        }
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

    fn create_test_order() -> Order {
        Order::new(1, Side::Buy, 100.25, 10).unwrap()
    }

    #[test]
    fn new_should_create_order() {
        let order = create_test_order();

        assert_eq!(order.id(), 1);
        assert_eq!(order.quantity(), 10);
        assert!(order.is_buy());
    }

    #[test]
    fn new_should_reject_invalid_price() {
        let result = Order::new(1, Side::Buy, 0.0, 10);

        assert_eq!(result.unwrap_err(), OrderError::InvalidPrice);
    }

    #[test]
    fn new_should_reject_nan_price() {
        let result = Order::new(1, Side::Buy, f64::NAN, 10);

        assert_eq!(result.unwrap_err(), OrderError::InvalidPrice);
    }

    #[test]
    fn new_should_reject_zero_quantity() {
        let result = Order::new(1, Side::Buy, 100.0, 0);

        assert_eq!(result.unwrap_err(), OrderError::InvalidQty);
    }

    #[test]
    fn buy_order_should_report_correct_side() {
        let order = create_test_order();

        assert!(order.is_buy());
        assert!(!order.is_sell());
    }

    #[test]
    fn reduce_should_decrease_quantity() {
        let mut order = create_test_order();

        let result = order.reduce(3);

        assert_eq!(result.unwrap(), 7);

        assert_eq!(order.quantity(), 7);
    }

    #[test]
    fn reduce_should_fail_when_amount_exceeds_quantity() {
        let mut order = create_test_order();

        let result = order.reduce(20);

        assert_eq!(
            result.unwrap_err(),
            OrderError::ReduceAmountExceedsRemaining
        );

        assert_eq!(order.quantity(), 10);
    }

    #[test]
    fn reduce_should_fail_when_amount_is_zero() {
        let mut order = create_test_order();

        let result = order.reduce(0);

        assert_eq!(result.unwrap_err(), OrderError::ZeroReduceAmount);

        assert_eq!(order.quantity(), 10);
    }

    #[test]
    fn reduce_all_quantity_should_mark_filled() {
        let mut order = create_test_order();

        let result = order.reduce(10);

        assert_eq!(result.unwrap(), 0);

        assert!(order.is_filled());
    }
}
