use protocol::{OrderRequest, OrderResponse};

pub trait OrderHandler {
    fn handle(&self, request: OrderRequest) -> OrderResponse;
}

#[derive(Debug, Default)]
pub struct SimpleOrderHandler;

impl OrderHandler for SimpleOrderHandler {
    fn handle(&self, request: OrderRequest) -> OrderResponse {
        match request {
            OrderRequest::AddOrder { id, .. } => OrderResponse::Accepted { id },

            OrderRequest::Book => OrderResponse::BookSnapshot,
        }
    }
}
