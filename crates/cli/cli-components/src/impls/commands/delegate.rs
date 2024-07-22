use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::traits::command::CommandRunner;
use crate::traits::output::HasOutputType;

pub struct DelegateCommandRunner<Components>(pub PhantomData<Components>);

impl<App, Args, Components, Delegate> CommandRunner<App, Args> for DelegateCommandRunner<Components>
where
    App: HasOutputType + HasErrorType,
    Components: DelegateComponent<Args, Delegate = Delegate>,
    Delegate: CommandRunner<App, Args>,
    Args: Async,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        Delegate::run_command(app, args).await
    }
}
