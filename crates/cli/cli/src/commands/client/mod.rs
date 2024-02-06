mod create;
pub use create::Create;

use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ClientCommands {
    /// Create a new client
    Create(Create),
}

impl ClientCommands {
    pub async fn run(&self, builder: CosmosBuilder) -> Result<()> {
        match self {
            ClientCommands::Create(cmd) => cmd.run(builder).await,
        }
    }
}
