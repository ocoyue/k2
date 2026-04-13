use thiserror::Error;

#[derive(Error,Debug,PartialEq)]
pub enum ParseErr {
    
    #[error("invalid line: {line}")]
    InvalidLine {line: String},

    #[error("Invalid order: {reason}")]
    InvalidOrder {reason : String},

    #[error("Order not found: {order_id}")]
    OrderNotFound {order_id : String},

    #[error("Invalid Side: {side}")]
    InvalidSide {side: String},
    
    #[error("Order already exists: {cmd}")]
    InvalidCommand {cmd: String},

}
#[derive(Error, Debug,PartialEq)]
pub enum ExecuteErr {
    #[error("Duplicate order id: {order_id}")]
    DuplicateOrderId { order_id : u32 },
    
    #[error("{0}")]
    Internal(String),
    
}


#[derive(Error, Debug,PartialEq)]
pub enum BusinessError {
    /// 解析阶段的错误
    #[error("Parse error: {0}")]
    Parse(#[from] ParseErr),        // #[from] 自动支持 ? 转换

    /// 执行命令阶段的错误
    #[error("Execute error: {0}")]
    Execute(#[from] ExecuteErr),

    // 可选：如果需要一个通用的内部错误
    #[error("Internal error: {0}")]
    Internal(String),
}