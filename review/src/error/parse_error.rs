use crate::error::order_error::OrderError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("invalid length of command")]
    InvalidLength,
    #[error("head command not found")]
    NoneHead,
    #[error("order side text is invalid")]
    SideText,
    #[error("order id text is invalid")]
    IdText,
    #[error("order price text is invalid")]
    PriceText,
    #[error("order quantity text is invalid")]
    QtyText,
    #[error("build order failed: {0}")]
    BuildOrderFailed(#[from] OrderError),
    #[error("unknown command")]
    UnknownCommand,
}
