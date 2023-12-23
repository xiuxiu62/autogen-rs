use thiserror::Error;

pub type OpenaiResult<T> = Result<T, OpenaiError>;

#[derive(Error, Debug)]
pub enum OpenaiError {
    #[error("Failed to convert `{0}` into an OpenaiEngine")]
    EngineSelection(String),
    #[error(transparent)]
    ChatGPT(#[from] chatgpt::err::Error),
    #[error(transparent)]
    ModelConfigurationBuilder(#[from] chatgpt::config::ModelConfigurationBuilderError),
}
