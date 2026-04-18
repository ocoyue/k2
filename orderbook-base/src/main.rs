mod engine;
mod error;
mod model;
mod parser;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::engine::*;
    use crate::error::*;
    use crate::model::*;
    use crate::parser::*;

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
        let cmd1 = parse_str(source_str).unwrap();
        assert_eq!(
            cmd1,
            Command::ADD(Order::new(108, 88.0, 100, Side::BUY).unwrap()),
        );
        assert_eq!(execute_cmd(cmd1, &mut orders), Ok(()));

        let cmd2: Command = parse_str(source_str).unwrap();
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
            parse_str(s3),
            Err(ParseErr::InvalidSide { side: err_info3 })
        );

        let s4 = "add,108,-88.0,100,buy";
        let err_info4 = "price must be positive".to_string();
        assert_eq!(
            parse_str(s4),
            Err(ParseErr::InvalidOrder { reason: err_info4 }),
        );
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
        let cmd1 = parse_str(source_str).unwrap();
        assert_eq!(cmd1, Command::CANCEL(3u32));
        assert_eq!(execute_cmd(cmd1, &mut orders), Ok(()));
        assert_eq!(orders.len(), 4);

        let cmd2 = parse_str("cancel , -3");
        let err_info2 = "invalid digit found in string".to_string();
        assert_eq!(cmd2, Err(ParseErr::Internal(err_info2)));

        println!("cancel_order -> Success");
    }
}
