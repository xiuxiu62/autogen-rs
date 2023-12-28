use super::error::{Error, Result};
use chatgpt::prelude::{ChatGPT, ChatGPTEngine, ModelConfigurationBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    model: String,
    api_key: Option<String>,
}

impl Config {
    pub fn new(name: &str, model: &str, api_key: Option<&str>) -> Self {
        Self {
            name: name.to_owned(),
            model: model.to_owned(),
            api_key: api_key.map(|key| key.to_owned()),
        }
    }

    pub fn as_client(&self) -> Result<ChatGPT> {
        let model_config = ModelConfigurationBuilder::default()
            .engine(OpenaiEngine::try_from(self.model.clone())?)
            .build()?;

        Ok(ChatGPT::new_with_config(
            self.api_key.as_ref().unwrap(),
            model_config,
        )?)
    }
}

#[repr(transparent)]
struct OpenaiEngine(ChatGPTEngine);

impl TryFrom<String> for OpenaiEngine {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let engine = match value.as_str() {
            "gpt-3.5-turbo" => Self(ChatGPTEngine::Gpt35Turbo),
            "gpt-3.5-turbo-0301" => Self(ChatGPTEngine::Gpt35Turbo_0301),
            "gpt-4.0" => Self(ChatGPTEngine::Gpt4),
            "gpt-4.0-32k" => Self(ChatGPTEngine::Gpt4_32k),
            "gpt-4.0-0314" => Self(ChatGPTEngine::Gpt4_0314),
            "gpt-4.0-32k-0314" => Self(ChatGPTEngine::Gpt4_32k_0314),
            _ => return Err(Error::EngineSelection(value)),
        };

        Ok(engine)
    }
}

impl From<OpenaiEngine> for ChatGPTEngine {
    fn from(value: OpenaiEngine) -> Self {
        value.0
    }
}
