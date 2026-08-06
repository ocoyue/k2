use gateway_01::engine::engine::Engine;
use gateway_01::engine::handle::EngineHandle;
use gateway_01::handler::router::HandlerRouter;
use gateway_01::session::Session;
use std::net::TcpListener;

fn main() {
    if let Err(e) = start() {
        eprintln!("{}", e);
    }
}
fn start() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;

    println!("server running");

    for stream in listener.incoming() {
        // Session::new(stream?, HandlerRouter::new()).run()?;
        let (tx, rx) = std::sync::mpsc::channel();

        let engine_handle = EngineHandle::new(tx);

        std::thread::spawn(move || {
            Engine::run(rx);
        });
    }

    Ok(())
}
