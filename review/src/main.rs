use review::engine::Engine;
use review::model::command::Command;
use review::model::order::Order;
use review::model::side::Side::Buy;
use review::state_machine::orderbook::OrderBook;
fn main() {
    let book = OrderBook::new();
    let mut engine: Engine = Engine::new(book);
    let o1 = Order::new(
        1,
        Buy,
        100.00,
        10
    ).unwrap();
    engine.execute(Command::Add(o1));
    engine.execute(Command::Get(1));
}

