use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();

    let mut reader = BufReader::new(stream.try_clone().unwrap());

    let lines = ["Tom", "Jack", "Jerry"];

    for line in lines {
        stream.write_all(format!("{}\n", line).as_bytes()).unwrap();

        let mut response = String::new();

        reader.read_line(&mut response).unwrap();

        print!("{}", response);
    }
}
