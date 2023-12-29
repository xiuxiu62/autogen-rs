use sophon_core::message::Message;
use std::fmt;
use thiserror::Error;
use tokio::{io, sync::mpsc};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    Io(#[from] io::Error),
    SendMessage(#[from] mpsc::error::SendError<Message>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
