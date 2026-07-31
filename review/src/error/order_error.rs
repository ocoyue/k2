use thiserror::Error;

#[derive(Debug,Error,Eq, PartialEq)]
pub enum OrderError{
    #[error("order id cannot be empty")]
    EmptyOrderId,
    #[error("price must be positive and finite")]
    NegativePrice,
    #[error("quantity must be greater than zero")]
    ZeroQuantity,
    #[error("reduce amount exceeds remaining quantity")]
    ReduceAmountExceedsRemaining ,
    #[error("reduce failed")]
    ReduceFailed
}