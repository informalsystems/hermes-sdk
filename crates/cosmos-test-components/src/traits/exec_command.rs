use std::path::Path;

use cgp_core::prelude::*;

pub struct ExecOutput {
    pub stdout: String,
    pub stderr: String,
}

#[async_trait]
pub trait CanExecCommand: HasErrorType {
    async fn exec_command(
        &self,
        description: &str,
        command_path: &Path,
        args: &[&str],
    ) -> Result<ExecOutput, Self::Error>;
}
