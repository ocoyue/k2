use crate::protocol::message::{Request, Response};

pub trait Handler {
    fn handle(&self, request: Request) -> Response;
}
