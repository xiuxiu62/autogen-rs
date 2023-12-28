use std::{borrow::Cow, sync::mpsc};

#[derive(Clone, Debug)]
pub struct Message<'agent> {
    id: u16,
    kind: MessageKind,
    contents: Cow<'agent, str>,
}

#[derive(Copy, Clone, Debug)]
pub enum MessageKind {
    User,
    Agent,
}

pub type MessagePublisher<'agent> = mpsc::Sender<Message<'agent>>;

pub type MessageSubscriber<'agent> = mpsc::Receiver<Message<'agent>>;
