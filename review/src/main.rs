use review::engine::Engine;
use review::model::command::Command;
use review::state_machine::orderbook::OrderBook;
fn main() {
    let book = OrderBook::new();
    let mut engine: Engine = Engine::new(book);
    let input1 = "ADD,1,BUY,100,10";
    let cmd1 =
        input1.parse::<Command>()
            .unwrap();
    let _ = engine.execute(cmd1);
    println!("end")



}
