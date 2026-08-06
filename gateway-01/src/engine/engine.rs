use std::sync::mpsc::Receiver;

use crate::{engine::request::EngineRequest, protocol::message::Response};
use crate::protocol::message::Request;

pub struct Engine;

impl Engine {
    pub fn run(receiver: Receiver<EngineRequest>) {
        for request in receiver {
            let response = Self::execute(request);

            println!("{:?}", response);
        }
    }

    fn execute(request: EngineRequest) -> Response {
        match request.req {
            Request::Hello { name } => Response::Greeting { name },
        }
    }
}
