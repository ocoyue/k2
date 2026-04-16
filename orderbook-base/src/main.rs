mod parser;
mod model;
mod engine;
mod error;
use crate::model::*;
use crate::error::*;
use crate::parser::*;
use crate::engine::*;
fn main() {
    run()
}
fn run(){
    add_test()
}
fn add_test() {
    let mut orders : Vec<Order> = vec![
        Order::new(1,88.0,100,Side::BUY).unwrap(),
        Order::new(2,88.0,100,Side::SELL).unwrap(),
        Order::new(3,88.0,100,Side::SELL).unwrap(),
        Order::new(4,88.0,100,Side::BUY).unwrap(),
        Order::new(5,88.0,100,Side::BUY).unwrap(),
    ];

    let s1 = "add,108,88,100,buy";
    let cmd1 : Command = parse_str(s1).unwrap();
    assert_eq!(
            Command::ADD(Order::new(108,88.0,100,Side::BUY).unwrap()),
            cmd1,
        );
    assert_eq!(execute_cmd(cmd1,&mut orders),Ok(()));

    let s2 = "add,1,88,100,buy";
    let cmd1 : Command = parse_str(s2).unwrap();
    assert_eq!(
            Command::ADD(Order::new(1,88.0,100,Side::BUY).unwrap()),
            cmd1,
        );
    assert_eq!(execute_cmd(cmd1,&mut orders),Err(ExecuteErr::DuplicateOrderId {order_id: 1}));


    let s3 = "add,108,88,100,buyx";
    assert!(matches!(
            parse_str(s3),
            Err(ParseErr::InvalidSide {side:_}),
        ));

    let s4 = "add,108,-88.0,100,buy";
    assert!(matches!(
            parse_str(s4),
            Err(ParseErr::InvalidOrder {reason:_}),
        ));





    // create orders
    // get &str
    // parse &str > command
    // execute command
    //
    //
    //
    //
    println!("end");
}