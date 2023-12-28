use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to convert `{0}` into an OpenaiEngine")]
    EngineSelection(String),
    #[error(transparent)]
    ChatGPT(#[from] chatgpt::err::Error),
    #[error(transparent)]
    ModelConfigurationBuilder(#[from] chatgpt::config::ModelConfigurationBuilderError),
}
