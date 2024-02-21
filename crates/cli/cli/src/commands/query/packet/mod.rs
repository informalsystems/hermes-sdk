mod pending;
pub use pending::QueryPendingPackets;

mod commitments;
pub use commitments::QueryPacketCommitments;

mod pending_acks;
pub use pending_acks::QueryPendingAcks;

mod util;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum PacketCommands {
    /// Query packet commitments
    Commitments(QueryPacketCommitments),

    /// Output a summary of pending packets in both directions of a channel
    Pending(QueryPendingPackets),

    /// Query packet acknowledgment
    PendingAcks(QueryPendingAcks),
}

impl CommandRunner<CosmosBuilder> for PacketCommands {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::Commitments(cmd) => cmd.run(builder).await,
            Self::Pending(cmd) => cmd.run(builder).await,
            Self::PendingAcks(cmd) => cmd.run(builder).await,
        }
    }
}
