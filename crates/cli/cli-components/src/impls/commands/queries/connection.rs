use cgp::prelude::*;

use crate::impls::QueryConnectionEndArgs;
use crate::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};

pub struct RunQueryConnectionSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum QueryConnectionSubCommand {
    /// Query the connection end
    End(QueryConnectionEndArgs),
}

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, QueryConnectionSubCommand> for RunQueryConnectionSubCommand
where
    App: CanRunCommand<QueryConnectionEndArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &QueryConnectionSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            QueryConnectionSubCommand::End(args) => app.run_command(args).await,
        }
    }
}
