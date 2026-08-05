use gateway_01::handler::MarketDataHandler;
use gateway_01::handler::hello_handler::HelloHandler;
use gateway_01::session::Session;
use std::net::TcpListener;
use gateway_01::handler::router::HandlerRouter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;

    println!("server running");

    for stream in listener.incoming() {
        let stream = stream?;
        Session::new(stream, HandlerRouter::new()).run()?;
    }

    Ok(())
}
