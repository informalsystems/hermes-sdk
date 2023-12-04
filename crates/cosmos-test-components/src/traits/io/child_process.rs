use cgp_core::prelude::*;

use crate::traits::types::io::child_process::HasChildProcessType;
use crate::traits::types::io::file_path::HasFilePathType;

#[derive_component(ChildProcessStarterComponent, ChildProcessStarter<Bootstrap>)]
#[async_trait]
pub trait CanStartChildProcess: HasChildProcessType + HasFilePathType + HasErrorType {
    async fn start_child_process(
        &self,
        command_path: &Self::FilePath,
        command_args: &[&str],
        stdout_path: Option<&Self::FilePath>,
        stderr_path: Option<&Self::FilePath>,
    ) -> Result<Self::ChildProcess, Self::Error>;
}
