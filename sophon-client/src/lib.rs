// struct AgentController<'agent> {
//     agent_id: u16,
//     publisher: MessagePublisher<'agent>,
// }

pub mod config {
    use serde::{Deserialize, Serialize};
    use std::{fmt, path::Path};
    use thiserror::Error;
    use tokio::{fs, io};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {}

    impl Config {
        pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
            Ok(toml::from_str(&fs::read_to_string(path).await?)?)
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        Deserialize(#[from] toml::de::Error),
        Io(#[from] io::Error),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{self:?}")
        }
    }
}
