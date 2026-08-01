use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum OrderError {
    #[error("price must be positive and finite")]
    InvalidPrice,
    #[error("quantity must be positive and finite")]
    InvalidQty,
    #[error("reduce amount exceeds remaining quantity")]
    ReduceAmountExceedsRemaining,
    #[error("reduce amount must be greater than zero")]
    ZeroReduceAmount,
}
