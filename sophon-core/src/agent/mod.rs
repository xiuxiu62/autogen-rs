use crate::{
    message::{Message, MessagePublisher, MessageSubscriber},
    Backend,
};
use std::{
    borrow::Cow,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockResult},
};
use tokio::sync::mpsc;

pub struct Agent<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    name: Cow<'agent, str>,
    inner: Arc<AgentInternal<B>>,
    publisher: Arc<MessagePublisher<'agent>>,
    subscriber: MessageSubscriber<'agent>,
}

impl<'agent, B> Agent<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    pub async fn query(&'agent self, message: Message) {
        self.try_write()
            .unwrap()
            .query(message, self.publisher.clone());
    }

    fn try_read(&self) -> TryLockResult<RwLockReadGuard<'_, B>> {
        self.inner.try_read()
    }

    fn try_write(&self) -> TryLockResult<RwLockWriteGuard<'_, B>> {
        self.inner.try_write()
    }
}

impl<'agent, B> Agent<'agent, B>
where
    for<'backend> B: Backend<'backend>,
{
    pub fn new(name: &'agent str, inner: B) -> Self {
        let (publisher, subscriber) = mpsc::unbounded_channel();

        Self {
            name: Cow::from(name),
            inner: Arc::new(AgentInternal::new(inner)),
            publisher: Arc::new(publisher),
            subscriber,
        }
    }
}

unsafe impl<'agent, B> Send for Agent<'agent, B> where for<'backend> B: Backend<'backend> {}
unsafe impl<'agent, B> Sync for Agent<'agent, B> where for<'backend> B: Backend<'backend> {}

struct AgentInternal<B>(RwLock<B>)
where
    for<'backend> B: Backend<'backend>;

impl<B> AgentInternal<B>
where
    for<'backend> B: Backend<'backend>,
{
    pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<'_, B>> {
        self.0.try_read()
    }

    pub fn try_write(&self) -> TryLockResult<RwLockWriteGuard<'_, B>> {
        self.0.try_write()
    }
}

impl<B> AgentInternal<B>
where
    for<'backend> B: Backend<'backend>,
{
    pub fn new(backend: B) -> Self {
        Self(RwLock::new(backend))
    }
}
