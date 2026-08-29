use orderbook_engine::{AddOrderResult, BookSnapshot, EngineProxy};

use protocol::{OrderRequest, OrderResponse, OrderView};

use std::future::Future;

pub trait OrderHandler {
    fn handle(&self, request: OrderRequest) -> impl Future<Output = OrderResponse> + Send;
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
    fn handle(&self, request: OrderRequest) -> impl Future<Output = OrderResponse> + Send {
        async move {
            match request {
                OrderRequest::AddOrder { id, symbol, qty } => {
                    let AddOrderResult { id, .. } = self.proxy.add_order(id, symbol, qty).await;
                    OrderResponse::Accepted { id }
                }

                OrderRequest::Book => {
                    let BookSnapshot { as_of_seq, orders } = self.proxy.get_book().await;
                    let orders_view = orders
                        .iter()
                        .map(|order| OrderView {
                            id: order.id(),
                            symbol: order.symbol().to_owned(),
                            qty: order.qty(),
                        })
                        .collect();
                    OrderResponse::BookSnapshot {
                        as_of_seq,
                        orders_view,
                    }
                }
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
