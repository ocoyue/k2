mod engine;
mod error;
mod model;
mod parser;
use crate::engine::*;
use crate::error::*;
use crate::model::*;
use std::fmt::{Debug, Display};
use std::str::FromStr;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_side() {
        assert_eq!(Side::from_str("buy ").unwrap(), Side::BUY);
        assert_eq!(Side::from_str("SELL ").unwrap(), Side::SELL);
        assert_eq!(
            Side::from_str("bye "),
            Err(ParseErr::InvalidSide {
                side: "bye ".to_string(),
            })
        );
    }

    #[test]
    fn add_order() {
        let o1 = Order::new(1, 88.0, 100, Side::BUY).unwrap();
        let o2 = Order::new(2, 88.0, 100, Side::SELL).unwrap();
        let o3 = Order::new(3, 88.0, 100, Side::SELL).unwrap();
        let o4 = Order::new(4, 88.0, 100, Side::BUY).unwrap();
        let o5 = Order::new(5, 88.0, 100, Side::BUY).unwrap();

        let mut orders = vec![o1, o2, o3, o4, o5];
        // This is so important when Debugging !
        // Notice the size of orders change between the point here or next line!
        // println!("{:?}", orders);

        let source_str = "add,108,88,100,buy";
        let cmd1 = Command::from_str(source_str).unwrap();
        assert_eq!(
            cmd1,
            Command::ADD(Order::new(108, 88.0, 100, Side::BUY).unwrap()),
        );
        assert_eq!(execute_cmd(cmd1, &mut orders), Ok(ExecuteResult::Added));

        let cmd2: Command = Command::from_str(source_str).unwrap();
        assert_eq!(
            cmd2,
            Command::ADD(Order::new(108, 88.0, 100, Side::BUY).unwrap()),
        );
        assert_eq!(
            execute_cmd(cmd2, &mut orders),
            Err(ExecuteErr::DuplicateOrderId { order_id: 108 })
        );

        let s3 = "add,108,88,100,buyx";
        let err_info3 = "buyx".to_string();
        assert_eq!(
            Command::from_str(s3),
            Err(ParseErr::InvalidSide { side: err_info3 })
        );

        let s4 = "add,108,-88.0,100,buy";
        assert_eq!(Command::from_str(s4), Err(ParseErr::InvalidPrice(-88.0)),);
        println!("add_order -> Success");
    }

    #[test]
    fn cancel_order() {
        let o1 = Order::new(1, 88.0, 100, Side::BUY).unwrap();
        let o2 = Order::new(2, 88.0, 100, Side::SELL).unwrap();
        let o3 = Order::new(3, 88.0, 100, Side::SELL).unwrap();
        let o4 = Order::new(4, 88.0, 100, Side::BUY).unwrap();
        let o5 = Order::new(5, 88.0, 100, Side::BUY).unwrap();

        let mut orders = vec![o1, o2, o3, o4, o5];

        // Input : "CANCEL , 3" , orders
        // Expect : orders.len() - 1
        let source_str = "cancel , 3";
        let cmd1 = Command::from_str(source_str).unwrap();
        assert_eq!(cmd1, Command::CANCEL(3u32));
        assert_eq!(execute_cmd(cmd1, &mut orders), Ok(ExecuteResult::Canceled));
        assert_eq!(orders.len(), 4);

        let cmd2 = Command::from_str("cancel , -3");
        let err_info2 = "invalid digit found in string".to_string();
        assert_eq!(cmd2, Err(ParseErr::Internal(err_info2)));

        // println!("orders = {:?}", orders);
        println!("cancel_order -> Success");
    }

    #[test]
    fn reduce_order() {
        let o1 = Order::new(1, 88.0, 100, Side::BUY).unwrap();
        let o2 = Order::new(2, 88.0, 100, Side::SELL).unwrap();
        let o3 = Order::new(3, 88.0, 100, Side::SELL).unwrap();
        let o4 = Order::new(4, 88.0, 100, Side::BUY).unwrap();
        let o5 = Order::new(5, 88.0, 100, Side::BUY).unwrap();

        let mut orders = vec![o1, o2, o3, o4, o5];
        // Input: "REDUCE,101,3"
        // Output: ExeResult

        let source_str1 = "REDUCE,1,50";
        let cmd1 = Command::from_str(source_str1);
        assert_eq!(cmd1, Ok(Command::REDUCE { id: 1, qty: 50 }));

        let resu1 = execute_cmd(cmd1.unwrap(), &mut orders);
        assert_eq!(resu1, Ok(ExecuteResult::Reduced));

        let source_str2 = "REDUCE,2,999";
        let cmd2 = Command::from_str(source_str2);
        assert_eq!(cmd2, Ok(Command::REDUCE { id: 2, qty: 999 }));

        let resu2 = execute_cmd(cmd2.unwrap(), &mut orders);
        assert_eq!(resu2, Err(ExecuteErr::QuantityNotEnough(999)));

        let source_str3 = "REDUCE,3,100";
        let cmd3 = Command::from_str(source_str3);
        assert_eq!(cmd3, Ok(Command::REDUCE { id: 3, qty: 100 }));

        let resu3 = execute_cmd(cmd3.unwrap(), &mut orders);
        assert_eq!(resu3, Ok(ExecuteResult::Deleted));

        let source_str4 = "REDUCE,400,100";
        let cmd4 = Command::from_str(source_str4);
        assert_eq!(cmd4, Ok(Command::REDUCE { id: 400, qty: 100 }));

        let resu4 = execute_cmd(cmd4.unwrap(), &mut orders);
        assert_eq!(resu4, Err(ExecuteErr::OrderNotFound { order_id: 400 }));

        // println!("orders = {:?}", orders);
        println!("reduce_order -> Success");
    }

    #[test]
    fn get_order() {
        // Input : "GET , 4"    Output : Order
        let o1 = Order::new(1, 88.0, 100, Side::BUY).unwrap();
        let o2 = Order::new(2, 88.0, 100, Side::SELL).unwrap();
        let o3 = Order::new(3, 88.0, 100, Side::SELL).unwrap();
        let o4 = Order::new(4, 88.0, 100, Side::BUY).unwrap();
        let o5 = Order::new(5, 88.0, 100, Side::BUY).unwrap();
        let mut orders = vec![o1, o2, o3, o4, o5];

        let source_str1 = "GET , 3";
        let cmd1 = Command::from_str(source_str1);
        assert_eq!(cmd1, Ok(Command::GET(3)));

        let resu1 = execute_cmd(cmd1.unwrap(), &mut orders);
        let reference_o = Order::new(3, 88.0, 100, Side::SELL).unwrap();
        assert_eq!(resu1, Ok(ExecuteResult::Order(reference_o)));

        let source_str2 = "GET , 2222";
        let cmd2 = Command::from_str(source_str2);
        assert_eq!(cmd2, Ok(Command::GET(2222)));

        let resu2 = execute_cmd(cmd2.unwrap(), &mut orders);
        assert_eq!(resu2, Err(ExecuteErr::OrderNotFound { order_id: 2222 }));

        let source_str2 = "GET , 2xx2";
        let cmd2 = Command::from_str(source_str2);
        assert_eq!(
            cmd2,
            Err(ParseErr::InvalidDigit(
                "invalid digit found in string".to_string()
            ))
        );
        println!("get_order -> Success");
    }

    #[test]
    fn show_summary() {
        let o1 = Order::new(1, 88.0, 100, Side::BUY).unwrap();
        let o2 = Order::new(2, 88.0, 100, Side::SELL).unwrap();
        let o3 = Order::new(3, 88.0, 100, Side::SELL).unwrap();
        let o4 = Order::new(4, 88.0, 100, Side::BUY).unwrap();
        let o5 = Order::new(5, 88.0, 100, Side::BUY).unwrap();
        let mut orders = vec![o1, o2, o3, o4, o5];
        // input : "SUMMARY" output : Summary
        let source_str = "summary";
        let cmd1 = Command::from_str(source_str).unwrap();
        let cmd2 = Command::from_str(source_str).unwrap();
        assert_eq!(cmd1, Command::SUMMARY);

        let smr = Summary {
            count: 5,
            buy_count: 3,
            sell_count: 2,
            total_value: 44000.0,
        };
        assert_eq!(
            execute_cmd(cmd1, &mut orders),
            Ok(ExecuteResult::Summary(smr))
        );

        if let Ok(resu) = execute_cmd(cmd2, &mut orders) {
            if let ExecuteResult::Summary(s) = resu {
                println!("{}", s);
            }
        }
        println!("show summary -> Success");
    }
}
