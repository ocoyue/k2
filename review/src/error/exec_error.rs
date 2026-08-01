use crate::error::orderbook_error::OrderBookError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExecErr {
    #[error(transparent)]
    OrderBook(#[from] OrderBookError),
}
