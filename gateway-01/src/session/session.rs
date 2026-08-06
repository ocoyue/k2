use crate::error::session_err::SessErr;
use crate::handler::Handler;
use crate::protocol::codec::{decode_request, encode_response};

use crate::engine::engine::Engine;
use crate::engine::handle::EngineHandle;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct Session<H>
where
    H: Handler,
{
    stream: TcpStream,

    handler: H,

    engine: EngineHandle,
}

impl<H> Session<H>
where
    H: Handler,
{
    pub fn new(stream: TcpStream, handler: H, engine: EngineHandle) -> Self {
        Self {
            stream,
            handler,
            engine,
        }
    }

    pub fn run(self) -> Result<(), SessErr> {
        let Self {
            stream,
            handler,
            engine,
        } = self;

        if let Ok(addr) = stream.peer_addr() {
            println!("Peer address: {addr}");
        }

        let mut reader = BufReader::new(stream.try_clone()?);
        let mut writer = stream;

        let mut frame = String::new();
        loop {
            frame.clear();
            let bytes_read = reader.read_line(&mut frame)?;

            if bytes_read == 0 {
                break;
            }

            let Some(request) = decode_request(&frame) else {
                continue;
            };

            println!("received: {}", frame.trim());

            let engine_request = handler.handle(request);

            engine.send(engine_request);
            // let encoded_response = encode_response(response);
            let encoded_response = "echo successful".to_string();
            writer.write_all(encoded_response.as_bytes())?;
            writer.flush()?;
        }

        Ok(())
    }
}
