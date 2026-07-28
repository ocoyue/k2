use thiserror::Error;

#[derive(Debug,Error)]
pub enum OrderError{
    #[error("order id cannot be empty")]
    EmptyOrderId,
    #[error("price must be positive and finite")]
    NegativePrice,
    #[error("quantity must be greater than zero")]
    ZeroQuantity,
    #[error("reduce amount must be greater than zero")]
    ReduceAmountZero,
    #[error("reduce amount {requested} exceeds remaining quantity {remaining}")]
    ReduceAmountExceedsRemaining {
        remaining: u64,
        requested: u64,
    },
}