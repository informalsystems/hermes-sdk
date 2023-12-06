use core::str;
use core::str::Utf8Error;
use std::ffi::OsStr;
use std::io::Error as IoError;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use ibc_test_components::runtime::traits::exec_command::{CommandExecutor, ExecOutput};
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;
use tokio::process::Command;

pub struct TokioExecCommand;

pub trait CanRaiseExecFailureError: HasErrorType {
    fn exec_failure_error(exit_code: Option<i32>, stdout: &str, stderr: &str) -> Self::Error;
}

#[async_trait]
impl<Runtime> CommandExecutor<Runtime> for TokioExecCommand
where
    Runtime: HasFilePathType
        + CanRaiseError<IoError>
        + CanRaiseError<Utf8Error>
        + CanRaiseExecFailureError,
    Runtime::FilePath: AsRef<OsStr>,
{
    async fn exec_command(
        _runtime: &Runtime,
        command_path: &Runtime::FilePath,
        args: &[&str],
    ) -> Result<ExecOutput, Runtime::Error> {
        let output = Command::new(command_path)
            .args(args)
            .output()
            .await
            .map_err(Runtime::raise_error)?;

        let stdout = str::from_utf8(&output.stdout).map_err(Runtime::raise_error)?;

        let stderr = str::from_utf8(&output.stderr).map_err(Runtime::raise_error)?;

        if output.status.success() {
            Ok(ExecOutput {
                stdout: stdout.to_owned(),
                stderr: stderr.to_owned(),
            })
        } else {
            Err(Runtime::exec_failure_error(
                output.status.code(),
                stdout,
                stderr,
            ))
        }
    }
}
