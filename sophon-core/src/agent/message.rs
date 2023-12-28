use std::sync::mpsc;

pub type MessagePublisher<'agent> = mpsc::Sender<Message>;
pub type MessageSubscriber<'agent> = mpsc::Receiver<Message>;

#[derive(Clone, Debug)]
pub struct Message {
    id: u16,
    // kind: MessageKind,
    contents: String,
}

impl Message {
    pub fn new(
        id: u16,
        // kind: MessageKind,
        contents: &str,
    ) -> Self {
        Self {
            id,
            // kind,
            contents: contents.to_owned(),
        }
    }
}

// #[derive(Copy, Clone, Debug)]
// pub enum MessageKind {
//     User,
//     Agent,
// }
