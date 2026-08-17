use crate::handler::OrderHandler;
use protocol::{OrderCodec, OrderResponse};
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

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
        println!("session thread: {:?}", thread::current().id());
        let mut reader = BufReader::new(self.stream.try_clone()?);
        let mut writer = self.stream;

        loop {
            let mut frame = String::new(); // 帧

            let size = reader.read_line(&mut frame)?;

            if size == 0 {
                println!("order client disconnected");
                break;
            }

            let req = OrderCodec::decode(frame.trim());

            let response = match req {
                Ok(request) => self.handler.handle(request),
                Err(message) => OrderResponse::Error { message },
            };

            let output = OrderCodec::encode(response);

            writer.write_all(output.as_bytes())?;
        }

        Ok(())
    }
}
