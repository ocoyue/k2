// the inputting is String , no matter what way from.

// " add ,1 ,buy,88.88,100 "

use crate::engine::executor::executor;
use crate::parser::parse_cmd;
use crate::state_machine::OrderBook;

pub fn session(s:String, ob: OrderBook)  {
    // parse
    let rst_cmd = parse_cmd(s);
    let cmd = match rst_cmd {
        Ok(cmd) => {
            // execute
            executor(cmd, ob )
        }
        Err(e ) => {
            println!("{}",e);
            return;

        }
    };
    //

}


