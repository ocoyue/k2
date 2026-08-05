use crate::error::session_err::SessErr;
use crate::handler::Handler;
use crate::protocol::codec::{decode_request, encode_response};

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct Session<H>
where
    H: Handler,
{
    stream: TcpStream,
    handler: H,
}

impl<H> Session<H>
where
    H: Handler,
{
    pub fn new(stream: TcpStream, handler: H) -> Self {
        Self { stream, handler }
    }

    pub fn run(self) -> Result<(), SessErr> {
        let Self { stream, handler } = self;

        if let Ok(addr) = stream.peer_addr() {
            println!("Peer address: {addr}");
        }

        let mut reader = BufReader::new(stream.try_clone()?);
        let mut writer = stream;

        loop {
            let mut frame = String::new();

            let bytes_read = reader.read_line(&mut frame)?;

            if bytes_read == 0 {
                break;
            }

            let Some(request) = decode_request(&frame) else {
                continue;
            };

            println!("received: {}", frame.trim());

            let response = handler.handle(request);

            let encoded_response = encode_response(response);

            writer.write_all(encoded_response.as_bytes())?;
            writer.flush()?;
        }

        Ok(())
    }
}
