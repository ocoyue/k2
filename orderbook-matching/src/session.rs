use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Sender};
use tokio::sync::oneshot;
use crate::model::command::EngineRequest;
use crate::model::{OrderbookCmd};
use crate::protocol::{fmt_exe_resu, fmt_parse_err};

pub(crate) async  fn run_session<R,W>(
    reader: R,
    // 函数接口中的 writer , 是抽象，传进来的可以是 stream, 内存buffer, 文件, stdout
    mut writer:  W,
    tx: Sender<EngineRequest>,
) -> io::Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();
    while let Some(line) = lines.next_line().await? {
        let output = handle_line(&line, tx.clone()).await;
        if !output.is_empty() {
            writer.write_all(output.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        };
    }
    Ok(())
}

async fn handle_line(line: &str, tx:Sender<EngineRequest>) -> String {
    if line.trim().is_empty() {
        return String::new();
    }
    match OrderbookCmd::from_str(line) {
        Err(e) => fmt_parse_err(e),
        Ok(cmd) => {
            let (reply_tx,reply_rx) = oneshot::channel();
            if tx.send(EngineRequest::new(cmd,reply_tx)).await.is_err() {
                return String::from("Err engine closed");
            }
            match reply_rx.await {
                Ok(result) => fmt_exe_resu(result),
                Err(_) => String::from("Err engine dropped reply"),
            }
        }
    }
}

pub(crate) async fn run_tcp_session(
    stream: TcpStream,
    addr: SocketAddr,
    tx: Sender<EngineRequest>,
) -> io::Result<()> {
    println!("tcp handler accepted connection from {}", addr);
    let (read_half, write_half) = stream.into_split();
    run_session(read_half,write_half,tx).await?;
    Ok(())
}
