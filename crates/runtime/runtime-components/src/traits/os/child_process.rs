use hermes_prelude::*;

use crate::traits::HasFilePathType;

#[cgp_component {
  name: ChildProcessTypeComponent,
  provider: ProvideChildProcessType,
  context: Runtime,
}]
pub trait HasChildProcessType: Async {
    type ChildProcess: Async;
}

pub type ChildProcessOf<Runtime> = <Runtime as HasChildProcessType>::ChildProcess;

#[cgp_component {
  provider: ChildProcessStarter,
  context: Runtime,
}]
#[async_trait]
pub trait CanStartChildProcess: HasChildProcessType + HasFilePathType + HasAsyncErrorType {
    async fn start_child_process(
        &self,
        command_path: &Self::FilePath,
        command_args: &[&str],
        envs: &[(&str, &str)],
        stdout_path: Option<&Self::FilePath>,
        stderr_path: Option<&Self::FilePath>,
    ) -> Result<Self::ChildProcess, Self::Error>;
}

#[cgp_component {
  provider: ChildProcessWaiter,
  context: Runtime,
}]
#[async_trait]
pub trait CanWaitChildProcess: HasChildProcessType + HasAsyncErrorType {
    async fn wait_child_process(child_process: Self::ChildProcess) -> Result<(), Self::Error>;
}
