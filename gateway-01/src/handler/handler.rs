use crate::engine::request::EngineRequest;
use crate::protocol::message::{Request};

pub trait Handler {
    fn handle(&self, request: Request) -> EngineRequest;
}
