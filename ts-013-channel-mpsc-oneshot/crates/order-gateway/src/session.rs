use crate::handler::OrderHandler;
use protocol::{OrderCodec, OrderResponse};
use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct OrderSession<H> {
    stream: TcpStream,
    handler: H,
}

impl<H> OrderSession<H>
where
    H: OrderHandler,
{
    pub fn new(stream: TcpStream, handler: H) -> Self {
        Self { stream, handler }
    }

    pub async fn run(self) -> io::Result<()> {
        let OrderSession {
            mut stream,
            handler,
        } = self;

        let (read_half, mut write_half) = stream.split();
        let mut reader = BufReader::new(read_half);

        loop {
            let mut frame = String::new();

            let size = reader.read_line(&mut frame).await?;

            if size == 0 {
                println!("order client disconnected");
                break;
            }

            let request_result = OrderCodec::decode(frame.trim());

            let response = match request_result {
                Ok(request) => handler.handle(request).await,
                Err(message) => OrderResponse::Error { message },
            };

            let output = OrderCodec::encode(response);

            write_half.write_all(output.as_bytes()).await?;
        }

        Ok(())
    }
}
