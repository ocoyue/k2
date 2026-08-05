// 本文件只作用：占用未来 Market Data 功能模块名字占位
use crate::protocol::message::{Request, Response};

use super::handler::Handler;

pub struct MarketDataHandler;

impl Handler for MarketDataHandler {
    fn handle(&self, request: Request) -> Response {
        match request {
            Request::Hello { name } => Response::Greeting { name },
        }
    }
}
