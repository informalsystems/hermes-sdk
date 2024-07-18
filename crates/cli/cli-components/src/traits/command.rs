use cgp_core::prelude::*;

#[derive_component(CommandRunnerComponent, CommandRunner<App>)]
#[async_trait]
pub trait CanRunCommand<Args>: HasErrorType
where
    Args: Async,
{
    async fn run_command(&self, args: &Args) -> Result<(), Self::Error>;
}
