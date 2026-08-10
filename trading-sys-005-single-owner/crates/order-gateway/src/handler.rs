use orderbook_engine::{AddOrderResult, BookSnapshot, EngineProxy};

use protocol::{OrderRequest, OrderResponse, OrderView};

pub trait OrderHandler {
    fn handle(&self, request: OrderRequest) -> OrderResponse;
}

#[derive(Debug)]
pub struct SimpleOrderHandler {
    proxy: EngineProxy,
}
impl SimpleOrderHandler {
    pub fn new(proxy: EngineProxy) -> Self {
        Self { proxy }
    }
}
impl OrderHandler for SimpleOrderHandler {
    fn handle(&self, request: OrderRequest) -> OrderResponse {
        match request {
            OrderRequest::AddOrder { id, symbol, qty } => {
                let AddOrderResult { id } = self.proxy.add_order(id, symbol, qty);
                OrderResponse::Accepted { id }
            }

            OrderRequest::Book => {
                let BookSnapshot { orders } = self.proxy.get_book();
                let orders_view = orders
                    .iter()
                    .map(|order| OrderView {
                        id: order.id(),
                        symbol: order.symbol().to_owned(),
                        qty: order.qty(),
                    })
                    .collect();
                OrderResponse::BookSnapshot { order_view: orders_view }
            }
        }
    }
}

// future example:
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct TradingOrderHandler;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RiskCheckingHandler;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MatchingHandler;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MockOrderHandler;
