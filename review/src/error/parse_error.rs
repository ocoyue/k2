use thiserror::Error;
#[derive(Debug,Error,PartialEq,Eq)]
pub enum ParseError {
    #[error("head command not found")]
    HeadText,
    #[error("order side text is invalid")]
    SideText,
    #[error("order id text is invalid")]
    IdText,
    #[error("order price text is invalid")]
    PriceText,
    #[error("order quantity text is invalid")]
    QtyText,
    #[error("build order failed")]
    BuildOrderFailed,

}