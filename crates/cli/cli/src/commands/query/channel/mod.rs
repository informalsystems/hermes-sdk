mod client;
mod ends;

pub use client::QueryChannelClient;
pub use ends::QueryChannelEnds;
use hermes_cli_components::impls::QueryChannelEndArgs;
use hermes_cli_components::traits::CanRunCommand;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::HermesApp;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum QueryChannel {
    End(QueryChannelEndArgs),
    Ends(QueryChannelEnds),
    Client(QueryChannelClient),
}

impl QueryChannel {
    pub async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::End(cmd) => app.run_command(cmd).await,
            Self::Ends(cmd) => cmd.run(app).await,
            Self::Client(cmd) => cmd.run(app).await,
        }
    }
}
