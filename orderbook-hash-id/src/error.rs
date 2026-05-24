use thiserror::Error;
use crate::model::orderbook::OrderId;

#[derive(Error, Debug, PartialEq)]
pub enum ParseErr {
    #[error("Invalid line: {line}")]
    InvalidLine { line: String },

    #[error("Invalid digit: {0}")]
    InvalidDigit(String),

    #[error("Invalid price: {0}")]
    InvalidPrice(f64),

    #[error("Invalid ticks: {0}")]
    InvalidTicks(i64),

    #[error("Invalid quantity: {0}")]
    InvalidQuantity(u32),

    #[error("Invalid parameters count: {line}")]
    InvalidParaCount { line: String },

    #[error("Invalid order: {reason}")]
    InvalidOrder { reason: String },

    #[error("Invalid Side: {side}")]
    InvalidSide { side: String },

    #[error("Invalid command: {cmd}")]
    InvalidCommand { cmd: String },
}
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ExeErr {
    #[error("Duplicate order id: {order_id}")]
    DuplicateOrderId { order_id: OrderId },

    #[error("Order not found: {order_id}")]
    OrderNotFound { order_id: OrderId },

    #[error("Quantity not enough: request={request} available={available}")]
    QuantityNotEnough { request: u32, available: u32 },
}
