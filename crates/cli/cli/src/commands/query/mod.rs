mod client;
pub use client::ClientCommands;

use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum QueryCommands {
    /// Query information about IBC clients
    #[clap(subcommand)]
    Clients(ClientCommands),
}

impl Runnable for QueryCommands {
    async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        match self {
            Self::Clients(cmd) => cmd.run(builder).await,
        }
    }
}
