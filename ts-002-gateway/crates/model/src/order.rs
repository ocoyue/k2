pub type OrderId = u64;
pub type ClientOrderId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Debug, Clone)]
pub struct Order {
    // 交易所内部id
    id: OrderId,
    // 客户端id
    client_id: ClientOrderId,
    // instrument_id:InstrumentId(ID更加节省复制资源）
    // instrument: Instrument,
    side: Side,
    order_type: OrderType,
    // tick价格
    price: i64,
    // 原始数量
    quantity: u64,
    // 已成交数量
    filled: u64,
}
