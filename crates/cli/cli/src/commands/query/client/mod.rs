mod state;
pub use state::QueryClientState;

mod status;
pub use status::QueryClientStatus;

mod consensus;
pub use consensus::QueryClientConsensus;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

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

impl CommandRunner<CosmosBuilder> for ClientCommands {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::State(cmd) => cmd.run(builder).await,
            Self::Status(cmd) => cmd.run(builder).await,
            Self::Consensus(cmd) => cmd.run(builder).await,
        }
    }
}
