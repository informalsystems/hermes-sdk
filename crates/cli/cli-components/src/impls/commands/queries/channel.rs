use crate::impls::commands::queries::channel_end::QueryChannelEndArgs;
use crate::traits::command::{CanRunCommand, CommandRunner};

pub struct RunQueryChannelSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum QueryChannelSubCommand {
    /// Query the channel end
    End(QueryChannelEndArgs),
}

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
