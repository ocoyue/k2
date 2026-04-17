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
        let mut orders: Vec<Order> = vec![
            Order::new(1, 88.0, 100, Side::BUY).unwrap(),
            Order::new(2, 88.0, 100, Side::SELL).unwrap(),
            Order::new(3, 88.0, 100, Side::SELL).unwrap(),
            Order::new(4, 88.0, 100, Side::BUY).unwrap(),
            Order::new(5, 88.0, 100, Side::BUY).unwrap(),
        ];

        // This is so important when Debugging !
        // Notice the size of orders change between the point here or next line!
        // println!("{:?}", orders);

        let source_str = "add,108,88,100,buy";
        let cmd1: Command = parse_str(source_str).unwrap();
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
        assert!(matches!(
            parse_str(s3),
            Err(ParseErr::InvalidSide { side: _ }),
        ));

        let s4 = "add,108,-88.0,100,buy";
        assert!(matches!(
            parse_str(s4),
            Err(ParseErr::InvalidOrder { reason: _ }),
        ));
        println!("end");
    }
}