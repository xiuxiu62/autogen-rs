use chatgpt::prelude::ChatGPT;
use sophon_core::message::{Message, MessagePublisher};
use std::sync::Arc;

mod config;
mod error;

pub use config::Config;
pub use error::{Error, Result};

#[derive(Debug)]
struct Backend {
    name: String,
    inner: ChatGPT,
}

impl TryFrom<Config> for Backend {
    type Error = Error;

    fn try_from(value: Config) -> Result<Self> {
        let inner = value.as_client()?;
        let name = value.name;

        Ok(Self { name, inner })
    }
}

impl<'backend> sophon_core::Backend<'backend> for Backend {
    async fn query(&'backend self, message: Message, publisher: Arc<MessagePublisher<'backend>>) {
        todo!()
    }
}
