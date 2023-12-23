mod config;
mod openai;

use crate::config::Config;
use tracing::debug;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> DynResult<()> {
    setup_tracing();

    let config: Config = toml::from_str(&std::fs::read_to_string("config.toml")?)?;

    debug!("\n{:#?}", config);
    debug!("\n{}", toml::to_string(&config)?);

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

trait Agent {}
