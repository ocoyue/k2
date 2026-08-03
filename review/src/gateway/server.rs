use std::io::{BufRead, BufReader, Write};

use std::net::TcpStream;

use crate::{engine::Engine, model::command::Command};

pub fn start_server(engine: Engine) {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();

    println!("server start");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("failed: {}", e);
            }
        }
    }
}
fn handle_connection(mut stream: TcpStream, engine: &mut Engine) {
    println!("handle_connection");
    let reader_stream = stream.try_clone().unwrap();
    let mut reader = BufReader::new(reader_stream);

    let mut writer = stream;
    loop {
        let mut line = String::new();

        let size = reader.read_line(&mut line).unwrap();

        if size == 0 {
            break;
        }
    }
}
