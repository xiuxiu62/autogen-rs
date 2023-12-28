use sophon_client::config::Config;
use tracing::debug;

#[tokio::main]
async fn main() -> error::Result<()> {
    setup_tracing();

    let config = Config::load("config.toml").await?;
    debug!("{:#?}", config);

    Ok(())
}

fn setup_tracing() {
    let max_level = if cfg!(debug_assertions) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(max_level)
        .with_target(false)
        .init();
}

pub mod error {
    use sophon_client::config;
    use std::fmt;
    use thiserror::Error;

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        Config(#[from] config::Error),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{self:?}")
        }
    }
}
