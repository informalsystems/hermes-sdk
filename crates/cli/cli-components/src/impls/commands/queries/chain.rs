use cgp::prelude::*;

use super::chain_status::QueryChainStatusArgs;
use crate::traits::command::{CanRunCommand, CommandRunner, CommandRunnerComponent};

pub struct RunQueryChainSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum QueryChainSubCommand {
    /// Query the chain status
    Status(QueryChainStatusArgs),
}

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, QueryChainSubCommand> for RunQueryChainSubCommand
where
    App: CanRunCommand<QueryChainStatusArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &QueryChainSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            QueryChainSubCommand::Status(args) => app.run_command(args).await,
        }
    }
}
