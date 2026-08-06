use crate::engine::request::EngineRequest;
use crate::protocol::message::{Request, Response};

use super::{Handler, HelloHandler};

pub struct HandlerRouter {
    hello: HelloHandler,
}

impl HandlerRouter {
    pub fn new() -> Self {
        Self {
            hello: HelloHandler,
        }
    }
}

impl Handler for HandlerRouter {
    fn handle(&self, request: Request) -> EngineRequest {
        self.hello.handle(request)
    }
}
