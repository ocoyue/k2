use review::model::command::Command;
use review::session::session;
use review::state_machine::orderbook::OrderBook;
fn main() {
    let ob = OrderBook::new();
    let s = String::from( "add,1,buy,88.8,10");
    session(s,ob);
    
}

