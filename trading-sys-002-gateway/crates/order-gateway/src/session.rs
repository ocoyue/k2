use crate::handler::OrderHandler;
use protocol::{OrderCodec, OrderResponse};
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

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

    pub fn run(self) -> io::Result<()> {
        let reader_stream = self.stream.try_clone()?;

        let mut reader = BufReader::new(reader_stream);
        let mut writer = self.stream;

        loop {
            let mut frame = String::new();

            let size = reader.read_line(&mut frame)?;

            if size == 0 {
                println!("order client disconnected");
                break;
            }

            let response = match OrderCodec::decode(frame.trim()) {
                Ok(request) => self.handler.handle(request),

                Err(message) => OrderResponse::Error { message },
            };

            let output = OrderCodec::encode(response);

            writer.write_all(output.as_bytes())?;
        }

        Ok(())
    }
}
