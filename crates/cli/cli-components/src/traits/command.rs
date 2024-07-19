use cgp_core::prelude::*;

use crate::traits::output::HasOutputType;

#[derive_component(CommandRunnerComponent, CommandRunner<App>)]
#[async_trait]
pub trait CanRunCommand<Args>: HasOutputType + HasErrorType
where
    Args: Async,
{
    async fn run_command(&self, args: &Args) -> Result<Self::Output, Self::Error>;
}
