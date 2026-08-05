use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessErr {
    #[error("session I/O error: {0}")]
    IoErr(#[from] io::Error),
}
