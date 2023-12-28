mod entity;
pub mod message;

use self::message::{Message, MessagePublisher, MessageSubscriber};
use std::{
    borrow::Cow,
    sync::{Arc, RwLock},
};

pub trait Backend<'a>: Send + Sync {
    fn query(&'a self, message: Message<'a>, publisher: Arc<MessagePublisher<'a>>);
}

pub struct Agent<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    name: Cow<'agent, str>,
    inner: Arc<AgentInner<B>>,
    publisher: Arc<MessagePublisher<'agent>>,
    subscriber: MessageSubscriber<'agent>,
}

impl<'agent, B> Agent<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    pub fn query(&self, message: Message<'agent>) {
        self.inner
            .0
            .write()
            .unwrap()
            .query(message, self.publisher.clone());
    }
}

impl<'agent, B> Agent<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    pub fn new(name: &str, inner: B) -> Self {
        let (publisher, subscriber) = std::sync::mpsc::channel();

        Self {
            name: Cow::from(name),
            inner: Arc::new(AgentInner::new(inner)),
            publisher: Arc::new(publisher),
            subscriber,
        }
    }
}

unsafe impl<'agent, B> Send for Agent<'agent, B> where for<'backend> B: Backend<'backend> {}
unsafe impl<'agent, B> Sync for Agent<'agent, B> where for<'backend> B: Backend<'backend> {}

struct AgentInner<B>(RwLock<B>)
where
    for<'backend> B: Backend<'backend>;

impl<B> AgentInner<B>
where
    for<'backend> B: Backend<'backend>,
{
    pub fn new(backend: B) -> Self {
        Self(RwLock::new(backend))
    }
}

// struct ExampleBackend;

// unsafe impl Send for ExampleBackend {}
// unsafe impl Sync for ExampleBackend {}

// impl<'a> Backend<'a> for ExampleBackend {
//     fn query(&'a self, message: Message<'a>, publisher: Arc<MessagePublisher<'a>>) {
//         todo!()
//     }
// }
