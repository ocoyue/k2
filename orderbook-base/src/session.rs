use crate::engine::execute_cmd;
use crate::model::{Command, Order, Side};
use crate::protocol::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
pub fn start_session() {
    println!("Starting session ...");
    run(&mut sample_orders());
    println!("Session end");
}
fn run(orders: &mut Vec<Order>) {
    let mut r = get_reader();
    loop {
        match (&mut r).lines().next() {
            None => break,
            Some(Ok(line)) => handle_line(&line, orders),
            Some(Err(e)) => println!("{}", e),
        }
    }
}
fn get_reader() -> BufReader<impl Read> {
    let file = File::open("./file/input.txt").expect("Err File not found");
    BufReader::new(file)
}

fn handle_line(line: &str, orders: &mut Vec<Order>) {
    if line.is_empty() {
        return;
    }

    let cmd1 = Command::from_str(line);
    if let Err(e) = cmd1 {
        wrt_parse_err(e);
        return;
    }
    let resu = execute_cmd(cmd1.unwrap(),orders);
    wrt_exe_resu(resu)
}
fn sample_orders() -> Vec<Order> {
    let o1 = Order::new(1, 88.0, 100, Side::Buy).unwrap();
    let o2 = Order::new(2, 88.0, 100, Side::Sell).unwrap();
    let o3 = Order::new(3, 88.0, 100, Side::Sell).unwrap();
    let o4 = Order::new(4, 88.0, 100, Side::Buy).unwrap();
    let o5 = Order::new(5, 88.0, 100, Side::Buy).unwrap();
    vec![o1, o2, o3, o4, o5]
}
