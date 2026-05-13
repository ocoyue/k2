use tokio::sync::oneshot;

#[derive(Debug)]
pub struct EngineRequest {
    pub cmd: EngineCmd,
    pub reply_tx: oneshot::Sender<EngineResponse>,
}

#[derive(Debug)]
pub enum EngineCmd {
    Ping,
    Echo(String),
    Quit,
    Unknown(String),
}
#[derive(Debug)]
pub enum EngineResponse {
    Pong,
    Echo(String),
    Bye,
    Err(String),
}
