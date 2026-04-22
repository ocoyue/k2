use crate::error::*;
use crate::model::*;

pub fn fmt_exe_resu(rst: Result<ExeResult, ExeErr>) -> String {
    match rst {
        Ok(ExeResult::Added) => "OK ADD".to_string(),
        Ok(ExeResult::Canceled) => "OK CANCELED".to_string(),
        Ok(ExeResult::Reduced) => "OK REDUCE".to_string(),
        Ok(ExeResult::Clear) => "OK REDUCE AND CLEAR".to_string(),
        Ok(ExeResult::Summary(s)) => format!("OK {}", s),
        Ok(ExeResult::Order(o)) => format!("OK {}", o),
        Err(exe_err) => format!("ERR {}", exe_err),
    }
}
pub fn fmt_parse_err(parse_err: ParseErr) -> String {
    format!("ERR {}", parse_err)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn exe_err_test() {
//         // {let e1 = ExeErr::OrderNotFound { order_id: 0 };
//         // wrt_exe_err(&mut out, e);}
//
//         let e2 = Err(ExeErr::QuantityNotEnough {
//             request: 999,
//             available: 100,
//         });
//         println!( fmt_exe_resu(e2));
//     }
// }
