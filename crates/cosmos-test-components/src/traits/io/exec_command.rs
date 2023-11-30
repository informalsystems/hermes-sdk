use cgp_core::prelude::*;

use crate::traits::types::file_path::HasFilePathType;

pub struct ExecOutput {
    pub stdout: String,
    pub stderr: String,
}

/// A context with capability to execute shell commands similar to shell scripts.
/// The result of a successful execution is stored as string.
#[async_trait]
pub trait CanExecCommand: HasFilePathType + HasErrorType {
    async fn exec_command(
        &self,
        description: &str,
        command_path: &Self::FilePath,
        args: &[&str],
    ) -> Result<ExecOutput, Self::Error>;
}
