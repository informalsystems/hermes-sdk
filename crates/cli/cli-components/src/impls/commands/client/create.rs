use cgp_core::error::HasErrorType;
use cgp_core::Async;

use crate::traits::build::{CanLoadBuilder, HasBuilderType};
use crate::traits::command::CommandRunner;
use crate::traits::output::HasOutputType;

pub struct RunCreateClient;

impl<App, Args, Build> CommandRunner<App, Args> for RunCreateClient
where
    App: HasOutputType + HasBuilderType<Builder = Build> + CanLoadBuilder + HasErrorType,
    Args: Async,
{
    async fn run_command(app: &App, _args: &Args) -> Result<App::Output, App::Error> {
        let _builder = app.load_builder().await?;

        todo!()
    }
}
