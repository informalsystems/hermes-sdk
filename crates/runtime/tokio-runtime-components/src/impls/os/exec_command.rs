use core::str;
use core::str::Utf8Error;
use std::ffi::OsStr;
use std::io::{Error as IoError, ErrorKind};

use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::exec_command::{CommandWithEnvsExecutor, ExecOutput};
use tokio::process::Command;

pub struct TokioExecCommand;

pub struct ExecCommandFailure {
    pub command: String,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub struct CommandNotFound {
    pub command: String,
}

impl<Runtime> CommandWithEnvsExecutor<Runtime> for TokioExecCommand
where
    Runtime: HasFilePathType
        + CanRaiseAsyncError<IoError>
        + CanRaiseAsyncError<Utf8Error>
        + CanRaiseAsyncError<CommandNotFound>
        + CanRaiseAsyncError<ExecCommandFailure>,
    Runtime::FilePath: AsRef<OsStr>,
{
    async fn exec_command_with_envs(
        _runtime: &Runtime,
        command_path: &Runtime::FilePath,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> Result<ExecOutput, Runtime::Error> {
        let output = Command::new(command_path)
            .args(args)
            .envs(Vec::from(envs))
            .kill_on_drop(true)
            .output()
            .await
            .map_err(|e| {
                if e.kind() == ErrorKind::NotFound {
                    Runtime::raise_error(CommandNotFound {
                        command: Runtime::file_path_to_string(command_path),
                    })
                } else {
                    Runtime::raise_error(e)
                }
            })?;

        let stdout = str::from_utf8(&output.stdout).map_err(Runtime::raise_error)?;

        let stderr = str::from_utf8(&output.stderr).map_err(Runtime::raise_error)?;

        if output.status.success() {
            Ok(ExecOutput {
                stdout: stdout.to_owned(),
                stderr: stderr.to_owned(),
            })
        } else {
            Err(Runtime::raise_error(ExecCommandFailure {
                command: Runtime::file_path_to_string(command_path),
                exit_code: output.status.code(),
                stdout: stdout.to_owned(),
                stderr: stderr.to_owned(),
            }))
        }
    }
}
