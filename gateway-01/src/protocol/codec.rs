use crate::protocol::message::{Request, Response};

pub fn decode_request(frame: &str) -> Option<Request> {
    let name = frame.trim();

    if name.is_empty() {
        return None;
    }

    Some(Request::Hello {
        name: name.to_string(),
    })
}

pub fn encode_response(response: Response) -> String {
    match response {
        Response::Greeting { name } => {
            format!("Hello {name}\n")
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_name_into_request() {
        let request = decode_request("Tom\n");

        assert_eq!(
            request,
            Some(Request::Hello {
                name: "Tom".to_string(),
            })
        );
    }

    #[test]
    fn encodes_greeting_response() {
        let response = Response::Greeting {
            name: "Jack".to_string(),
        };

        let encoded = encode_response(response);

        assert_eq!(encoded, "Hello Jack\n");
    }

    #[test]
    fn ignores_empty_frame() {
        assert_eq!(decode_request("\n"), None);
    }
}