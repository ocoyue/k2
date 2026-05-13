use crate::model::EngineCmd::*;
use crate::model::{EngineRequest, EngineResponse};
use tokio::io;
use tokio::sync::mpsc::Receiver;

pub(crate) async fn engine_loop(mut rx: Receiver<EngineRequest>) -> io::Result<()> {
    while let Some(req) = rx.recv().await {
        let response = match req.cmd {
            Ping => EngineResponse::Pong,
            Echo(s) => EngineResponse::Echo(s),
            Quit => EngineResponse::Bye,
            Unknown(s) => EngineResponse::Err(format!("unknown command: {}", s)),
        };

        if let Err(_) = req.reply_tx.send(response){
            eprintln!("the receiver dropped");
        }
    }
    Ok(())
}
