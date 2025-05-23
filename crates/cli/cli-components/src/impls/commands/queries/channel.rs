use hermes_prelude::*;

use crate::impls::QueryChannelEndArgs;
use crate::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};

pub struct RunQueryChannelSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum QueryChannelSubCommand {
    /// Query the channel end
    End(QueryChannelEndArgs),
}

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, QueryChannelSubCommand> for RunQueryChannelSubCommand
where
    App: CanRunCommand<QueryChannelEndArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &QueryChannelSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            QueryChannelSubCommand::End(args) => app.run_command(args).await,
        }
    }
}
