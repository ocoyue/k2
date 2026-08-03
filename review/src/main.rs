use review::engine::Engine;
use review::gateway::server::start_server;
use review::state_machine::OrderBook;

fn main() {
    let engine =
        Engine::new(
            OrderBook::new()
        );


    start_server(engine);
    // let mut engine = Engine::new(OrderBook::new());
    //
    // let inputs = vec![
    //     "ADD,1,BUY,100,10",
    //     "GET,1",
    //     "REDUCE,1,3",
    //     "SUMMARY",
    //     "CANCEL,1",
    //     "SUMMARY",
    // ];
    //
    // for input in inputs {
    //     let cmd = input.parse::<Command>().unwrap();
    //
    //     let result = engine.execute(cmd);
    //
    //     println!("{:?}", result);
    // }
}
