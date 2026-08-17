use orderbook_engine::BookService;

use protocol::{OrderRequest, OrderResponse, OrderView};

pub trait OrderHandler {
    fn handle(&mut self, request: OrderRequest) -> OrderResponse;
}

#[derive(Debug)]
pub struct SimpleOrderHandler {
    service: BookService,
}
impl SimpleOrderHandler {
    pub fn new(service: BookService) -> Self {
        Self { service }
    }
}
impl OrderHandler for SimpleOrderHandler {
    fn handle(&mut self, request: OrderRequest) -> OrderResponse {
        match request {
            OrderRequest::AddOrder { id, symbol, qty } => {
                let result = self.service.add_order(id, symbol, qty);

                OrderResponse::Accepted { id: result.id }
            }

            OrderRequest::Book => {
                let snapshot = self.service.get_book();

                let orders = snapshot
                    .orders
                    .into_iter()
                    .map(|order| OrderView {
                        id: order.id(),
                        symbol: order.symbol().to_owned(),
                        qty: order.qty(),
                    })
                    .collect();

                OrderResponse::BookSnapshot { orders }
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
