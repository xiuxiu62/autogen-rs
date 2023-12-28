// use crate::DynResult;
// use serde::{Deserialize, Serialize};
// use std::path::Path;

// #[derive(Deserialize, Serialize, Debug)]
// pub struct Config {
//     #[serde(rename = "openai")]
//     openai_agents: Vec<OpenaiConfig>,
// }

// impl Config {
//     // fn new(openai_agents: Vec<OpenaiConfig>) -> Self {
//     //     Self { openai_agents }
//     // }

//     pub fn load<P: AsRef<Path>>(path: P) -> DynResult<Self> {
//         Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
//     }
// }
