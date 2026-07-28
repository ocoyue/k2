// engine -> control the whole lifecycle of OrderBook

// input: Command output: Result

use crate::model::command::Command;
use crate::state_machine::OrderBook;

pub(crate) fn executor(ob_cmd: Command, mut book: OrderBook) -> u32 {
    match ob_cmd {
        Command::Add(o) => {
            book.add(o);
            book.print_all();
            0
        }
        Command::Get(n) => {
            if let Some(o) = book.get(n){
                println!("id : {}", o.id);
            }else {
                println!("there is no order that has id={}", n);
            }
            0
        }
    }
}
// fn orderbook_exist() -> bool {
//     todo!()
// }
// fn get_book() -> OrderBook {
//     if orderbook_exist() {
//         todo!() // sync order book from catch
//     } else {
//         OrderBook::new()
//     }
// }

