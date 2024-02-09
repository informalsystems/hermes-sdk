mod end;
pub use end::QueryConnectionEnd;

use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum QueryConnection {
    /// Create a new channel
    End(QueryConnectionEnd),
}

impl QueryConnection {
    pub async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        match self {
            Self::End(cmd) => cmd.run(builder).await,
        }
    }
}
