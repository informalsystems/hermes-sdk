mod client;
mod end;
mod ends;

pub use client::QueryChannelClient;
pub use end::QueryChannelEnd;
pub use ends::QueryChannelEnds;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum QueryChannel {
    End(QueryChannelEnd),
    Ends(QueryChannelEnds),
    Client(QueryChannelClient),
}

impl QueryChannel {
    pub async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self {
            Self::End(cmd) => cmd.run(builder).await,
            Self::Ends(cmd) => cmd.run(builder).await,
            Self::Client(cmd) => cmd.run(builder).await,
        }
    }
}
