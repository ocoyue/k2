mod engine;
mod error;
mod model;
mod parser;
mod protocol;
mod session;

use crate::engine::run_orderbook_engine;
use crate::model::command::OrderbookReq;
use crate::model::orderbook::OrderBook;
use crate::model::{Order, Side};
use crate::session::run_session;
use std::io::BufReader;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread::spawn;

fn main() -> std::io::Result<()> {
    let mut orderbook = init_orderbook();
    let mut stream = init_tcp_stream()?;
    let (tx, rx) = init_mpsc_channel();
    let reader = BufReader::new(stream.try_clone()?);

    spawn(move || {
        run_orderbook_engine(&mut orderbook, rx);
    });
    run_session(reader, &mut stream, tx)?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn init_mpsc_channel() -> (mpsc::Sender<OrderbookReq>, mpsc::Receiver<OrderbookReq>) {
    mpsc::channel::<OrderbookReq>()
}
fn init_tcp_stream() -> std::io::Result<TcpStream> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;
    let (stream, _) = listener.accept()?;
    Ok(stream)
}
fn init_orderbook() -> OrderBook {
    let o1 = Order::new(1, 88.0, 100, Side::Buy).unwrap();
    let o2 = Order::new(2, 88.0, 100, Side::Sell).unwrap();
    let o3 = Order::new(3, 88.0, 100, Side::Sell).unwrap();
    let o4 = Order::new(4, 88.0, 100, Side::Buy).unwrap();
    let o5 = Order::new(5, 88.0, 100, Side::Buy).unwrap();

    OrderBook::from_orders(vec![o1, o2, o3, o4, o5])
}

#[cfg(test)]
mod tests {
    use crate::engine::*;
    use crate::error::*;
    use crate::model::orderbook::OrderBook;
    use crate::model::*;
    use crate::protocol::*;
    use std::str::FromStr;

    fn sample_orderbook() -> OrderBook {
        let o1 = Order::new(1, 88.0, 100, Side::Buy).unwrap();
        let o2 = Order::new(2, 88.0, 100, Side::Sell).unwrap();
        let o3 = Order::new(3, 88.0, 100, Side::Sell).unwrap();
        let o4 = Order::new(4, 88.0, 100, Side::Buy).unwrap();
        let o5 = Order::new(5, 88.0, 100, Side::Buy).unwrap();
        OrderBook::from_orders(vec![o1, o2, o3, o4, o5])
    }

    #[test]
    fn test_parse_side() {
        assert_eq!(Side::from_str("buy ").unwrap(), Side::Buy);
        assert_eq!(Side::from_str("SELL ").unwrap(), Side::Sell);
        assert_eq!(
            Side::from_str("bye "),
            Err(ParseErr::InvalidSide {
                side: "bye ".to_string(),
            })
        );
    }

    #[test]
    fn add_order() {
        let mut orderbook = sample_orderbook();
        // This is so important when Debugging !
        // Notice the size of orderbook size change between the point here or next line!
        // println!("{:?}", orderbook);

        let source_str = "add,108,88,100,buy";
        let cmd1 = OrderbookCmd::from_str(source_str).unwrap();
        assert_eq!(
            cmd1,
            OrderbookCmd::Add(Order::new(108, 88.0, 100, Side::Buy).unwrap()),
        );
        assert_eq!(execute_cmd(cmd1, &mut orderbook), Ok(ExeOk::Added));

        let cmd2: OrderbookCmd = OrderbookCmd::from_str(source_str).unwrap();
        assert_eq!(
            cmd2,
            OrderbookCmd::Add(Order::new(108, 88.0, 100, Side::Buy).unwrap()),
        );
        assert_eq!(
            execute_cmd(cmd2, &mut orderbook),
            Err(ExeErr::DuplicateOrderId { order_id: 108 })
        );

        let s3 = "add,108,88,100,buyx";
        let err_info3 = "buyx".to_string();
        assert_eq!(
            OrderbookCmd::from_str(s3),
            Err(ParseErr::InvalidSide { side: err_info3 })
        );

        let s4 = "add,108,-88.0,100,buy";
        assert_eq!(
            OrderbookCmd::from_str(s4),
            Err(ParseErr::InvalidPrice(-88.0)),
        );
        println!("add_order -> Success");
    }

    #[test]
    fn cancel_order() {
        let mut orderbook = sample_orderbook();

        let source_str = "cancel , 3";
        let cmd1 = OrderbookCmd::from_str(source_str).unwrap();
        assert_eq!(cmd1, OrderbookCmd::Cancel(3u32));
        assert_eq!(execute_cmd(cmd1, &mut orderbook), Ok(ExeOk::Canceled));
        assert_eq!(orderbook.len(), 4);

        let cmd2 = OrderbookCmd::from_str("cancel , -3");
        let err_info2 = "invalid digit found in string".to_string();
        assert_eq!(cmd2, Err(ParseErr::InvalidDigit(err_info2)));

        println!("cancel_order -> Success");
    }

    #[test]
    fn reduce_order() {
        let mut orderbook = sample_orderbook();
        // Input: "REDUCE,101,3"
        // Output: ExeResult

        let source_str1 = "REDUCE,1,50";
        let cmd1 = OrderbookCmd::from_str(source_str1);
        assert_eq!(cmd1, Ok(OrderbookCmd::Reduce { id: 1, qty: 50 }));

        let resu1 = execute_cmd(cmd1.unwrap(), &mut orderbook);
        assert_eq!(resu1, Ok(ExeOk::Reduced));

        let source_str2 = "REDUCE,2,999";
        let cmd2 = OrderbookCmd::from_str(source_str2);
        assert_eq!(cmd2, Ok(OrderbookCmd::Reduce { id: 2, qty: 999 }));

        let resu2 = execute_cmd(cmd2.unwrap(), &mut orderbook);
        assert_eq!(
            resu2,
            Err(ExeErr::QuantityNotEnough {
                request: 999,
                available: 100
            })
        );

        let source_str3 = "REDUCE,3,100";
        let cmd3 = OrderbookCmd::from_str(source_str3);
        assert_eq!(cmd3, Ok(OrderbookCmd::Reduce { id: 3, qty: 100 }));

        let resu3 = execute_cmd(cmd3.unwrap(), &mut orderbook);
        assert_eq!(resu3, Ok(ExeOk::Clear));

        let source_str4 = "REDUCE,400,100";
        let cmd4 = OrderbookCmd::from_str(source_str4);
        assert_eq!(cmd4, Ok(OrderbookCmd::Reduce { id: 400, qty: 100 }));

        let resu4 = execute_cmd(cmd4.unwrap(), &mut orderbook);
        assert_eq!(resu4, Err(ExeErr::OrderNotFound { order_id: 400 }));

        println!("reduce_order -> Success");
    }

    #[test]
    fn get_order() {
        let mut orderbook = sample_orderbook();
        // Input : "GET , 4"    Output : Order

        let source_str1 = "GET , 3";
        let cmd1 = OrderbookCmd::from_str(source_str1);
        assert_eq!(cmd1, Ok(OrderbookCmd::Get(3)));

        let resu1 = execute_cmd(cmd1.unwrap(), &mut orderbook);
        let reference_o = Order::new(3, 88.0, 100, Side::Sell).unwrap();
        assert_eq!(resu1, Ok(ExeOk::Order(reference_o)));

        let source_str2 = "GET , 2222";
        let cmd2 = OrderbookCmd::from_str(source_str2);
        assert_eq!(cmd2, Ok(OrderbookCmd::Get(2222)));

        let resu2 = execute_cmd(cmd2.unwrap(), &mut orderbook);
        assert_eq!(resu2, Err(ExeErr::OrderNotFound { order_id: 2222 }));

        let source_str2 = "GET , 2xx2";
        let cmd2 = OrderbookCmd::from_str(source_str2);
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
        let mut orderbook = sample_orderbook();
        // input : "SUMMARY" output : Summary
        let source_str = "summary";
        let cmd1 = OrderbookCmd::from_str(source_str).unwrap();
        let cmd2 = OrderbookCmd::from_str(source_str).unwrap();
        assert_eq!(cmd1, OrderbookCmd::Summary);

        let smr = Summary {
            orders_count: 5,
            buy_count: 3,
            sell_count: 2,
            total_value: 44000.0,
        };
        assert_eq!(execute_cmd(cmd1, &mut orderbook), Ok(ExeOk::Summary(smr)));

        if let Ok(ExeOk::Summary(s)) = execute_cmd(cmd2, &mut orderbook) {
            println!("{}", s);
        }

        println!("show summary -> Success");
    }

    #[test]
    fn wrt_test() {
        let mut orderbook = sample_orderbook();

        let source_str = "add,108,88,100,buy";
        let cmd1 = OrderbookCmd::from_str(source_str).unwrap();
        let resu1: Result<ExeOk, ExeErr> = execute_cmd(cmd1, &mut orderbook);

        fmt_exe_resu(resu1);

        let resu2 = ExeOk::Summary(Summary {
            orders_count: 5,
            buy_count: 3,
            sell_count: 2,
            total_value: 44000.0,
        });

        fmt_exe_resu(Ok(resu2));

        let o = Order::new(3, 88.0, 100, Side::Sell).unwrap();
        let resu3 = ExeOk::Order(o);
        fmt_exe_resu(Ok(resu3));

        println!("write result -> Success");
    }
}
