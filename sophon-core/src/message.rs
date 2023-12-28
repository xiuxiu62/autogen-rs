use crate::entity::EntityId;
use std::sync::mpsc;

pub type MessagePublisher<'agent> = mpsc::Sender<Message>;
pub type MessageSubscriber<'agent> = mpsc::Receiver<Message>;

#[derive(Clone, Debug)]
pub struct Message {
    pub sender: EntityId,
    pub contents: String,
}

impl Message {
    pub fn new(sender: EntityId, contents: &str) -> Self {
        Self {
            sender,
            contents: contents.to_owned(),
        }
    }
}
