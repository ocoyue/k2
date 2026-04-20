use crate::error::*;
use crate::model::*;
use std::io::{self, Write};

pub fn wrt_resu(rst: Result<ExeResult, BusinessErr>) {
    let mut out = io::stdout();
    match rst {
        Ok(exe_result) => wrt_exe_result(&mut out, exe_result),
        Err(bn_err) => writeln!(out, "ERR {}", bn_err).unwrap(),
    }
}
fn wrt_exe_result(out: &mut impl Write, exe_result: ExeResult) {
    match exe_result {
        ExeResult::Added => writeln!(out, "OK ADD").unwrap(),
        ExeResult::Canceled => writeln!(out, "OK CANCELED").unwrap(),
        ExeResult::Reduced => writeln!(out, "OK REDUCE").unwrap(),
        ExeResult::Clear => writeln!(out, "OK REDUCE AND CLEAR").unwrap(),
        ExeResult::Summary(s) => writeln!(out, "OK {}", s).unwrap(),
        ExeResult::Order(o) => writeln!(out, "OK {}", o).unwrap(),
    }
}

fn wrt_parse_err(out: &mut impl Write, parse_err: ParseErr) {
    match parse_err {
        ParseErr::InvalidLine { line } => writeln!(out, "ERR {}", line).unwrap(),
        ParseErr::InvalidDigit(s) => writeln!(out, "ERR {}", s).unwrap(),
        ParseErr::InvalidPrice(f) => writeln!(out, "ERR {}", f).unwrap(),
        ParseErr::InvalidQuantity(q) => writeln!(out, "ERR {}", q).unwrap(),

        ParseErr::InvalidParaCount { line } => writeln!(out, "ERR {}", line).unwrap(),
        ParseErr::InvalidOrder { reason } => writeln!(out, "ERR {}", reason).unwrap(),
        ParseErr::InvalidSide { side } => writeln!(out, "ERR {}", side).unwrap(),
        ParseErr::InvalidCommand { cmd } => writeln!(out, "ERR {}", cmd).unwrap(),
    }
}

fn wrt_exe_err(out: &mut impl Write, exe_err: ExeErr) {
    match exe_err {
        ExeErr::DuplicateOrderId { order_id } => writeln!(out, "ERR {}", order_id).unwrap(),
        ExeErr::OrderNotFound { .. } => writeln!(out, "ERR {}", exe_err).unwrap(),
        ExeErr::QuantityNotEnough { .. } => writeln!(out, "ERR {}", exe_err).unwrap(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exe_err_test() {
        // {let e1 = ExeErr::OrderNotFound { order_id: 0 };
        // wrt_exe_err(&mut out, e);}

        let e2 = Err(BusinessErr::ExeErr(ExeErr::QuantityNotEnough {
            request: 999,
            available: 100,
        }));
        wrt_resu(e2)
    }
}
