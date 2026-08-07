#[derive(Debug, PartialEq, Eq)]
pub enum OrderRequest {
    AddOrder { id: u64, symbol: String, qty: u64 },

    Book,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderResponse {
    Accepted { id: u64 },

    BookSnapshot,

    Error { message: String },
}

pub struct OrderCodec;

impl OrderCodec {
    pub fn decode(input: &str) -> Result<OrderRequest, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["ADD", id, symbol, qty] => {
                let id = id
                    .parse::<u64>()
                    .map_err(|_| "invalid order id".to_string())?;

                let qty = qty
                    .parse::<u64>()
                    .map_err(|_| "invalid quantity".to_string())?;

                Ok(OrderRequest::AddOrder {
                    id,
                    symbol: symbol.to_string(),
                    qty,
                })
            }

            ["BOOK"] => Ok(OrderRequest::Book),

            [] => Err("empty order request".to_string()),

            _ => Err("unknown or malformed order request".to_string()),
        }
    }

    pub fn encode(response: OrderResponse) -> String {
        match response {
            OrderResponse::Accepted { id } => {
                format!("ACCEPTED id={id}\n")
            }

            OrderResponse::BookSnapshot => "BOOK count=0 orders=[]\n".to_string(),

            OrderResponse::Error { message } => {
                format!("ERR {message}\n")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OrderCodec, OrderRequest, OrderResponse};

    #[test]
    fn decode_add_order() {
        let result = OrderCodec::decode("ADD 1 BTCUSDT 10").unwrap();

        assert_eq!(
            result,
            OrderRequest::AddOrder {
                id: 1,
                symbol: "BTCUSDT".to_string(),
                qty: 10,
            }
        );
    }

    #[test]
    fn decode_book() {
        let result = OrderCodec::decode("BOOK").unwrap();

        assert_eq!(result, OrderRequest::Book);
    }

    #[test]
    fn reject_invalid_order_id() {
        let result = OrderCodec::decode("ADD abc BTCUSDT 10");

        assert_eq!(result, Err("invalid order id".to_string()));
    }

    #[test]
    fn encode_accepted() {
        let result = OrderCodec::encode(OrderResponse::Accepted { id: 7 });

        assert_eq!(result, "ACCEPTED id=7\n");
    }
}
