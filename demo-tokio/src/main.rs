mod engine;
mod model;
mod session;
mod tcp;

use crate::engine::engine_loop;
use crate::model::EngineRequest;
use tcp::*;
use tokio::sync::mpsc::channel;
use tokio::{io, spawn};
use crate::session::session::tcp_handle_session;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = init_tcp().await?;
    println!("Listening on {}", listener.local_addr()?);

    let (tx, rx) = channel::<EngineRequest>(1024);
    spawn(async move {
        if let Err(e) = engine_loop(rx).await {
            eprintln!("Engine loop exited with error: {}", e);
        }
    });
    tcp_loop(listener, move |stream, addr| {
        let tx = tx.clone();
        async move { tcp_handle_session(stream, addr, tx).await }
    })
    .await?;
    Ok(())
}
