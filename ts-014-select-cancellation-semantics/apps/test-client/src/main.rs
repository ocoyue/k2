use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let address = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9000".to_string());

    let mut stream = TcpStream::connect(&address).expect("failed to connect");

    println!("connected to {address}");

    let reader_stream = stream.try_clone().expect("failed to clone stream");

    let mut reader = BufReader::new(reader_stream);

    loop {
        let mut input = String::new();

        let size = io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");

        if size == 0 {
            break;
        }

        stream
            .write_all(input.as_bytes())
            .expect("failed to write request");

        let mut response = String::new();

        let size = reader
            .read_line(&mut response)
            .expect("failed to read response");

        if size == 0 {
            println!("server disconnected");
            break;
        }

        print!("{response}");
    }
}
