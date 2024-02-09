mod state;
use hermes_cli_framework::output::Output;
pub use state::QueryClientState;

use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ClientCommands {
    /// Query the state of a client
    State(QueryClientState),
}

impl Runnable for ClientCommands {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        match self {
            Self::State(cmd) => cmd.run(builder).await,
        }
    }
}
