mod end;
pub use end::QueryConnectionEnd;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum QueryConnection {
    /// Create a new channel
    End(QueryConnectionEnd),
}

impl CommandRunner<HermesApp> for QueryConnection {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        match self {
            Self::End(cmd) => cmd.run(app).await,
        }
    }
}
