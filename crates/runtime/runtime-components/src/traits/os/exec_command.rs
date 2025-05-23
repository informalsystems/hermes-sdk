use alloc::string::String;

use hermes_prelude::*;

use crate::traits::HasFilePathType;

#[derive(Debug)]
pub struct ExecOutput {
    pub stdout: String,
    pub stderr: String,
}

/// A context with capability to execute shell commands similar to shell scripts.
/// The result of a successful execution is stored as string.
#[cgp_component {
  provider: CommandExecutor,
  context: Runtime,
}]
#[async_trait]
pub trait CanExecCommand: HasFilePathType + HasAsyncErrorType {
    async fn exec_command(
        &self,
        command_path: &Self::FilePath,
        args: &[&str],
    ) -> Result<ExecOutput, Self::Error>;
}

/// A context with capability to execute shell commands similar to shell scripts.
/// The result of a successful execution is stored as string.
#[cgp_component {
  provider: CommandWithEnvsExecutor,
  context: Runtime,
}]
#[async_trait]
pub trait CanExecCommandWithEnvs: HasFilePathType + HasAsyncErrorType {
    async fn exec_command_with_envs(
        &self,
        command_path: &Self::FilePath,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> Result<ExecOutput, Self::Error>;
}
