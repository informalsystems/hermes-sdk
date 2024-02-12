use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

mod create;
pub use create::ClientCreate;

mod update;
pub use update::ClientUpdate;

#[derive(Debug, clap::Subcommand)]
pub enum ClientCommands {
    /// Create a new client
    Create(ClientCreate),

    /// Update a client
    Update(ClientUpdate),
}

impl CommandRunner for ClientCommands {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        match self {
            Self::Create(cmd) => cmd.run(builder).await,
            Self::Update(cmd) => cmd.run(builder).await,
        }
    }
}
