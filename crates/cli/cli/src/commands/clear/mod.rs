mod packets;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
pub use packets::PacketsClear;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ClearCommands {
    /// Clear pending packets
    Packets(PacketsClear),
}

impl CommandRunner<CosmosBuilder> for ClearCommands {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::Packets(cmd) => cmd.run(builder).await,
        }
    }
}
