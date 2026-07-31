use crate::error::order_error::OrderError;
use thiserror::Error;

#[derive(Debug, PartialEq,Eq,Error)]
pub enum OrderBookError {
    #[error("duplicated id: {0}")]
    DuplicateId(u32),

    #[error("order not found: {0}")]
    NotFound(u32),

    #[error("Order Error: {0}")]
    OrderError(OrderError),

}