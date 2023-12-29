use message::{Message, MessagePublisher};
use std::sync::Arc;

pub mod agent;
pub mod entity;
pub mod message;
// pub mod config;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait Backend<'a>: Send + Sync {
    type Error: std::error::Error;

    async fn query(
        &'a mut self,
        message: Message,
        publisher: Arc<MessagePublisher<'a>>,
    ) -> Result<(), Self::Error>;
}
