use cgp_core::error::HasErrorType;
use cgp_core::Async;

use crate::traits::build::{CanLoadBuilder, HasBuilderType};
use crate::traits::command::CommandRunner;

pub struct RunCreateClient;

impl<App, Args, Build> CommandRunner<App, Args> for RunCreateClient
where
    App: HasBuilderType<Builder = Build> + CanLoadBuilder + HasErrorType,
    Args: Async,
{
    async fn run_command(app: &App, _args: &Args) -> Result<(), App::Error> {
        let _builder = app.load_builder().await?;

        // TODO

        Ok(())
    }
}
