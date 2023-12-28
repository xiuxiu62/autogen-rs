use autogen_core::config::Config;
use tracing::debug;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> DynResult<()> {
    setup_tracing();

    let config = Config::load("config.toml")?;
    debug!("\n{:#?}", config);

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

struct AgentController {
    agent_id: u16,
    publisher: MessagePublisher,
    // kind: AgentKind,
}

// impl AgentController {
//     fn new(agent_id: u16, kind: AgentKind) -> Self {
//         Self { agent_id, kind }
//     }
// }

// enum AgentKind {
//     Process,
//     Consensus,
//     Manager,
// }
