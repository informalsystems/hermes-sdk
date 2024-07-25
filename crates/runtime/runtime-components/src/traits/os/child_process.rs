use cgp_core::prelude::*;

use crate::traits::fs::file_path::HasFilePathType;

#[derive_component(ChildProcessTypeComponent, ProvideChildProcessType<Runtime>)]
pub trait HasChildProcessType: Async {
    type ChildProcess: Async;
}

pub type ChildProcessOf<Runtime> = <Runtime as HasChildProcessType>::ChildProcess;

#[derive_component(ChildProcessStarterComponent, ChildProcessStarter<Runtime>)]
#[async_trait]
pub trait CanStartChildProcess: HasChildProcessType + HasFilePathType + HasErrorType {
    async fn start_child_process(
        &self,
        command_path: &Self::FilePath,
        command_args: &[&str],
        envs: &[(&str, &str)],
        stdout_path: Option<&Self::FilePath>,
        stderr_path: Option<&Self::FilePath>,
    ) -> Result<Self::ChildProcess, Self::Error>;
}

#[derive_component(ChildProcessWaiterComponent, ChildProcessWaiter<Runtime>)]
#[async_trait]
pub trait CanWaitChildProcess: HasChildProcessType + HasErrorType {
    async fn wait_child_process(child_process: Self::ChildProcess) -> Result<(), Self::Error>;
}
