use crate::model::{EngineCmd, EngineRequest, EngineResponse};
use crate::session::session_ctx::SessionContext;
use std::net::SocketAddr;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
pub(crate) async fn run_session<R, W>(
    reader: R,
    writer: W,
    tx: Sender<EngineRequest>,
    ctx: SessionContext,
) -> io::Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    match ctx {
        SessionContext::Stream { addr } => stream_session(reader, writer, tx, addr).await,
        SessionContext::File { path: _ } => Ok(()),
        SessionContext::Std => Ok(()),
    }
}
async fn stream_session<R, W>(
    reader: R,
    mut writer: W,
    tx: Sender<EngineRequest>,
    addr: SocketAddr,
) -> io::Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let buf_reader = BufReader::new(reader);

    let mut lines = buf_reader.lines();
    while let Some(line) = lines.next_line().await? {
        println!("from {}: {:?}", addr, line);
        let cmd = parse_command(&line);
        let should_close = matches!(cmd, EngineCmd::Quit);
        let (reply_tx, reply_rx) = oneshot::channel::<EngineResponse>();

        if let Err(e) = tx.send(EngineRequest { cmd, reply_tx }).await {
            eprintln!("failed to send Engine Request: {}", e);
            writer.write_all(b"ERR engine closed\n").await?;
            break;
        }
        match reply_rx.await {
            Ok(response) => {
                writer
                    .write_all(format_response(response).as_bytes())
                    .await?;
            }
            Err(_) => {
                writer.write_all(b"ERR engine dropped reply\n").await?;
                break;
            }
        };
        if should_close {
            break;
        }
    }
    Ok(())
}
pub fn parse_command(line: &str) -> EngineCmd {
    if line == "PING" {
        EngineCmd::Ping
    } else if line == "QUIT" {
        EngineCmd::Quit
    } else if let Some(rest) = line.strip_prefix("ECHO ") {
        EngineCmd::Echo(rest.to_string())
    } else {
        EngineCmd::Unknown(line.to_string())
    }
}
pub fn format_response(response: EngineResponse) -> String {
    match response {
        EngineResponse::Pong => "PONG\n".to_string(),
        EngineResponse::Echo(s) => format!("{}\n", s),
        EngineResponse::Bye => "BYE\n".to_string(),
        EngineResponse::Err(e) => format!("ERR {}\n", e),
    }
}
pub(crate) async fn tcp_handle_session(
    stream: TcpStream,
    addr: SocketAddr,
    tx: Sender<EngineRequest>,
) -> io::Result<()> {
    println!("client connected: {}", addr);
    let (read_half, write_half) = stream.into_split();

    let result = run_session(read_half, write_half, tx, SessionContext::Stream { addr }).await;

    println!("client disconnected: {}", addr);
    result
}
