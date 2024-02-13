mod packets;
pub use packets::PacketsClear;

use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum ClearCommands {
    /// Clear pending packets
    Packets(PacketsClear),
}

impl Runnable for ClearCommands {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        match self {
            Self::Packets(cmd) => cmd.run(builder).await,
        }
    }
}
