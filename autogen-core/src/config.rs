use crate::openai::config::OpenaiConfig;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(rename = "openai")]
    openai_agents: Vec<OpenaiConfig>,
}

impl Config {
    fn new(openai_agents: Vec<OpenaiConfig>) -> Self {
        Self { openai_agents }
    }
}
