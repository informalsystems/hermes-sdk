use alloc::string::String;

use cgp::prelude::*;

use crate::traits::fs::file_path::HasFilePathType;

#[derive(Debug)]
pub struct ExecOutput {
    pub stdout: String,
    pub stderr: String,
}

/// A context with capability to execute shell commands similar to shell scripts.
/// The result of a successful execution is stored as string.
#[derive_component(CommandExecutorComponent, CommandExecutor<Runtime>)]
#[async_trait]
pub trait CanExecCommand: HasFilePathType + HasErrorType {
    async fn exec_command(
        &self,
        command_path: &Self::FilePath,
        args: &[&str],
    ) -> Result<ExecOutput, Self::Error>;
}

/// A context with capability to execute shell commands similar to shell scripts.
/// The result of a successful execution is stored as string.
#[derive_component(CommandWithEnvsExecutorComponent, CommandWithEnvsExecutor<Runtime>)]
#[async_trait]
pub trait CanExecCommandWithEnvs: HasFilePathType + HasErrorType {
    async fn exec_command_with_envs(
        &self,
        command_path: &Self::FilePath,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> Result<ExecOutput, Self::Error>;
}
