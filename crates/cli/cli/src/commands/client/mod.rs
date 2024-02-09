mod create;
pub use create::ClientCreate;

use hermes_cli_framework::command::Runnable;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ClientCommands {
    /// Create a new client
    Create(ClientCreate),
}

impl Runnable for ClientCommands {
    async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        match self {
            Self::Create(cmd) => cmd.run(builder).await,
        }
    }
}
