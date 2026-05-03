use crate::engine::execute_cmd;
use crate::model::Command;
use crate::model::orderbook::OrderBook;
use crate::protocol::*;
use std::io;
use std::io::{BufRead, Write};
use std::str::FromStr;
pub fn run_session<R: BufRead, W: Write>(
    reader: R,
    // 函数接口中的 writer , 是抽象，传进来的可以是 stream, 内存buffer, 文件, stdout 
    writer: &mut W,
    orderbook: &mut OrderBook,
) -> io::Result<()> {
    for line_resu in reader.lines() {
        let s = line_resu?;
        let output = handle_line(&s, orderbook);
        if !output.is_empty() {
            writeln!(writer, "{output}")?;
        };
    }
    Ok(())
}

fn handle_line(line: &str, orderbook: &mut OrderBook) -> String {
    if line.trim().is_empty() {
        return String::new();
    }
    match Command::from_str(line) {
        Err(e) => fmt_parse_err(e),
        Ok(cmd) => fmt_exe_resu(execute_cmd(cmd, orderbook)),
    }
}
