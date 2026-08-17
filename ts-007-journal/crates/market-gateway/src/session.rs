use crate::handler::MarketDataHandler;
use protocol::{MarketDataCodec, MarketDataResponse};
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct MarketSession<H> {
    stream: TcpStream,
    handler: H,
}

impl<H> MarketSession<H>
where
    H: MarketDataHandler,
{
    pub fn new(stream: TcpStream, handler: H) -> Self {
        Self { stream, handler }
    }

    pub fn run(mut self) -> io::Result<()> {
        let reader_stream = self.stream.try_clone()?;

        let mut reader = BufReader::new(reader_stream);

        loop {
            let mut frame = String::new();

            let size = reader.read_line(&mut frame)?;

            if size == 0 {
                println!("market client disconnected");
                break;
            }

            let response = match MarketDataCodec::decode(frame.trim()) {
                Ok(request) => self.handler.handle(request),

                Err(message) => MarketDataResponse::Error { message },
            };

            let output = MarketDataCodec::encode(response);

            self.stream.write_all(output.as_bytes())?;
        }

        Ok(())
    }
}
