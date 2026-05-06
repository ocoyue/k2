use crate::model::{ExeOk, OrderbookCmd};
use crate::protocol::*;
use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use crate::error::ExeErr;
use crate::model::command::EngineRequest;

pub fn run_session<R: BufRead, W: Write>(
    reader: R,
    // 函数接口中的 writer , 是抽象，传进来的可以是 stream, 内存buffer, 文件, stdout
    writer: &mut W,
    tx: Sender<EngineRequest>,
    // done_tx: SyncSender<()>,
) -> io::Result<()> {
    for line_resu in reader.lines() {
        let s = line_resu?;
        let output = handle_line(&s, tx.clone());
        if !output.is_empty() {
            writeln!(writer, "{output}")?;
            writer.flush()?;
        };
        // if output.eq("OK SHUTDOWN"){
        //     done_tx.send(()).unwrap();
        //     break;
        // }
    }
    Ok(())
}

fn handle_line(line: &str, tx:Sender<EngineRequest>) -> String {
    if line.trim().is_empty() {
        return String::new();
    }
    match OrderbookCmd::from_str(line) {
        Err(e) => fmt_parse_err(e),
        Ok(cmd) => {
            let (reply,reply_re) = mpsc::channel::<Result<ExeOk, ExeErr>>();
            tx.send(EngineRequest::new(cmd, reply)).unwrap();
            fmt_exe_resu(reply_re.recv().unwrap())
        }
    }
}
