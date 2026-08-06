use crate::engine::request::EngineRequest;
use crate::handler::handler::Handler;
use crate::protocol::message::{Request, Response};

pub struct HelloHandler;
impl HelloHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for HelloHandler {
    fn handle(&self, request: Request) -> EngineRequest {
        EngineRequest{req:request}
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_tom_request() {
        let handler = HelloHandler::new();

        let request = Request::Hello {
            name: "Tom".to_string(),
        };

        let response = handler.handle(request);

        assert_eq!(
            response, 
            EngineRequest { req: Request::Hello { name: "Tom".to_string() }, }
        );
    }

    #[test]
    fn handles_jack_request() {
        let handler = HelloHandler::new();

        let request = Request::Hello {
            name: "Jack".to_string(),
        };

        let response = handler.handle(request);

        assert_eq!(
            response,
            EngineRequest { req: Request::Hello { name: "Jack".to_string() }, }

        );
    }
}