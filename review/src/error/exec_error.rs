use thiserror::Error;
use crate::error::orderbook_error::OrderBookError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExecErr {

    #[error("OrderBook Error: {0}")]
    OrderBookError(OrderBookError),
    #[error("reduce amount must be greater than zero")]
    ReduceAmountZero,
    #[error("amount must be greater than quantity")]
    AmountGreaterQuantity{
        request:u64,
        qty:u64,
    },
    #[error("reduce failed")]
    ReduceFailed,
}
