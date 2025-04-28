use hermes_prelude::*;

use crate::impls::{QueryClientStateArgs, QueryClientStatusArgs, QueryConsensusStateArgs};
use crate::traits::{CanRunCommand, CommandRunner, CommandRunnerComponent};

pub struct RunQueryClientSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum QueryClientSubCommand {
    /// Query the state of a client
    State(QueryClientStateArgs),

    /// Query the status of a client
    Status(QueryClientStatusArgs),

    /// Query the consensus state of a client
    Consensus(QueryConsensusStateArgs),
}

#[cgp_provider(CommandRunnerComponent)]
impl<App> CommandRunner<App, QueryClientSubCommand> for RunQueryClientSubCommand
where
    App: CanRunCommand<QueryClientStateArgs>
        + CanRunCommand<QueryClientStatusArgs>
        + CanRunCommand<QueryConsensusStateArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &QueryClientSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            QueryClientSubCommand::State(args) => app.run_command(args).await,
            QueryClientSubCommand::Status(args) => app.run_command(args).await,
            QueryClientSubCommand::Consensus(args) => app.run_command(args).await,
        }
    }
}
