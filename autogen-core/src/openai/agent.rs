use super::{
    config::OpenaiConfig,
    error::{OpenaiError, OpenaiResult},
};
use chatgpt::prelude::ChatGPT;

#[derive(Debug)]
pub struct OpenaiAgent {
    name: String,
    inner: ChatGPT,
}

impl TryFrom<OpenaiConfig> for OpenaiAgent {
    type Error = OpenaiError;

    fn try_from(value: OpenaiConfig) -> OpenaiResult<Self> {
        let inner = value.as_client()?;
        let name = value.name;

        Ok(Self { name, inner })
    }
}
