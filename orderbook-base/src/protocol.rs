use crate::error::*;
use crate::model::*;
use std::io::{self, Write};

pub fn wrt_exe_resu(rst: Result<ExeResult, ExeErr>) {
    let mut out = io::stdout();
    match rst {
        Ok(ExeResult::Added) => writeln!(out, "OK ADD").unwrap(),
        Ok(ExeResult::Canceled) => writeln!(out, "OK CANCELED").unwrap(),
        Ok(ExeResult::Reduced) => writeln!(out, "OK REDUCE").unwrap(),
        Ok(ExeResult::Clear) => writeln!(out, "OK REDUCE AND CLEAR").unwrap(),
        Ok(ExeResult::Summary(s)) => writeln!(out, "OK GET {}", s).unwrap(),
        Ok(ExeResult::Order(o)) => writeln!(out, "OK GET {}", o).unwrap(),
        Err(exe_err) => writeln!(out, "ERR {}", exe_err).unwrap(),
    }
}
pub fn wrt_parse_err(parse_err:ParseErr) {
    writeln!(io::stdout(), "ERR {}", parse_err).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exe_err_test() {
        // {let e1 = ExeErr::OrderNotFound { order_id: 0 };
        // wrt_exe_err(&mut out, e);}

        let e2 = Err(ExeErr::QuantityNotEnough {
            request: 999,
            available: 100,
        });
        wrt_exe_resu(e2)
    }
}
