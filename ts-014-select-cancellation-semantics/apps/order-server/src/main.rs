use order_gateway::{OrderSession, SimpleOrderHandler};
use orderbook_engine::{recover_engine_state, start_engine_loop};
use tokio::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:9000";
const JOURNAL_PATH: &str = "data/order.journal";
const SNAPSHOT_PATH: &str = "data/order.snapshot";

const CHECKPOINT_INTERVAL: u64 = 2;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 1. Recovery Phase
    let recovered =
        recover_engine_state(JOURNAL_PATH, SNAPSHOT_PATH).expect("failed to recover engine state");

    // 2. Runtime Phase
    let engine_proxy =
        start_engine_loop(recovered, JOURNAL_PATH, SNAPSHOT_PATH, CHECKPOINT_INTERVAL)
            .expect("failed to start engine loop");

    // 3. Network Bootstrap
    let listener = TcpListener::bind(ADDRESS)
        .await
        .expect("failed to bind order server");

    println!("order server listening on {ADDRESS}");

    // 4. Accept
    loop {
        match listener.accept().await {
            Ok((stream, address)) => {
                // 5. Session
                println!("order client connected: {address}");

                let proxy = engine_proxy.clone();

                let handler = SimpleOrderHandler::new(proxy);
                let session = OrderSession::new(stream, handler);

                tokio::spawn(async move {
                    if let Err(error) = session.run().await {
                        eprintln!("order session error: {error}");
                    }
                });
            }
            Err(error) => {
                eprintln!("accept error: {error}");
            }
        }
    }
}
