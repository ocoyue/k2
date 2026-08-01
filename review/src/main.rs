use review::engine::Engine;
use review::model::command::Command;
use review::state_machine::orderbook::OrderBook;
fn main() {
    let mut engine = Engine::new(OrderBook::new());

    let inputs = vec![
        "ADD,1,BUY,100,10",
        "GET,1",
        "REDUCE,1,3",
        "SUMMARY",
        "CANCEL,1",
        "SUMMARY",
    ];

    for input in inputs {
        let cmd = input.parse::<Command>().unwrap();

        let result = engine.execute(cmd);

        println!("{:?}", result);
    }
}
