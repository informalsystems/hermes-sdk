mod state;
pub use state::QueryClientState;

mod status;
pub use status::QueryClientStatus;

mod consensus;
pub use consensus::QueryClientConsensus;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ClientCommands {
    /// Query the state of a client
    State(QueryClientState),

    /// Query the status of a client
    Status(QueryClientStatus),

    /// Query the consensus state of a client
    Consensus(QueryClientConsensus),
}

impl CommandRunner<HermesApp> for ClientCommands {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::State(cmd) => cmd.run(app).await,
            Self::Status(cmd) => cmd.run(app).await,
            Self::Consensus(cmd) => cmd.run(app).await,
        }
    }
}
