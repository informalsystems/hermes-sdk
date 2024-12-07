use cgp::core::component::UseDelegate;
use cgp::prelude::*;

use crate::traits::output::HasOutputType;

#[cgp_component {
  name: CommandRunnerComponent,
  provider: CommandRunner,
  context: App,
}]
#[async_trait]
pub trait CanRunCommand<Args>: HasOutputType + HasErrorType
where
    Args: Async,
{
    async fn run_command(&self, args: &Args) -> Result<Self::Output, Self::Error>;
}

impl<App, Args, Components, Delegate> CommandRunner<App, Args> for UseDelegate<Components>
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
