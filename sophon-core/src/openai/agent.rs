use crate::agent::{
    message::{Message, MessagePublisher},
    Backend,
};

use super::{
    config::OpenaiConfig,
    error::{OpenaiError, OpenaiResult},
};
use chatgpt::prelude::ChatGPT;

#[derive(Debug)]
pub struct OpenaiBackend {
    name: String,
    inner: ChatGPT,
}

impl TryFrom<OpenaiConfig> for OpenaiBackend {
    type Error = OpenaiError;

    fn try_from(value: OpenaiConfig) -> OpenaiResult<Self> {
        let inner = value.as_client()?;
        let name = value.name;

        Ok(Self { name, inner })
    }
}

impl<'backend> Backend<'backend> for OpenaiBackend {
    fn query(
        &'backend self,
        message: Message<'backend>,
        publisher: std::sync::Arc<MessagePublisher<'backend>>,
    ) {
        todo!()
    }
}
