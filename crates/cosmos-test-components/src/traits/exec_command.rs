use cgp_core::prelude::*;

use crate::traits::file_path::HasFilePathType;

pub struct ExecOutput {
    pub stdout: String,
    pub stderr: String,
}

#[async_trait]
pub trait CanExecCommand: HasFilePathType + HasErrorType {
    async fn exec_command(
        &self,
        description: &str,
        command_path: &Self::FilePath,
        args: &[&str],
    ) -> Result<ExecOutput, Self::Error>;
}
