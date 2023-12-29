use std::fmt;
use tokio::sync::mpsc;

pub type MessagePublisher<'agent> = mpsc::UnboundedSender<Message>;
pub type MessageSubscriber<'agent> = mpsc::UnboundedReceiver<Message>;

#[derive(Clone, Debug)]
pub struct Message {
    pub sender: String,
    pub contents: String,
}

impl Message {
    pub fn new(sender: String, contents: String) -> Self {
        Self { sender, contents }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n{}", self.sender, self.contents)
    }
}
