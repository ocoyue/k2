use crate::engine::execute_cmd;
use crate::model::{Command, Order};
use crate::protocol::*;
use std::io::{BufRead, Write};
use std::str::FromStr;

pub fn run_session<R: BufRead, W: Write>(reader: R, writer: &mut W, orders: &mut Vec<Order>) {
    for line_resu in reader.lines() {
        match line_resu {
            Ok(s) => {
                let output = handle_line(&s, orders);
                if !output.is_empty() {
                    writeln!(writer, "{output}").unwrap();
                }
            }
            Err(e) => writeln!(writer, "ERR {e}").unwrap(),
        }
    }
}

fn handle_line(line: &str, orders: &mut Vec<Order>) -> String {
    if line.trim().is_empty() {
        return String::new();
    }
    match Command::from_str(line) {
        Err(e) => fmt_parse_err(e),
        Ok(cmd) => fmt_exe_resu(execute_cmd(cmd, orders)),
    }
}
