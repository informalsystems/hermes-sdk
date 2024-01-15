use core::time::Duration;
use std::io::Error as IoError;
use std::path::Path;
use std::process::{ExitStatus, Stdio};

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_async_runtime_components::task::types::future_task::FutureTask;
use hermes_relayer_components::runtime::traits::sleep::CanSleep;
use hermes_relayer_components_extra::runtime::traits::spawn::CanSpawnTask;
use hermes_test_components::runtime::traits::child_process::ChildProcessStarter;
use hermes_test_components::runtime::traits::read_file::CanReadFileAsString;
use hermes_test_components::runtime::traits::types::child_process::HasChildProcessType;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use tokio::fs::OpenOptions;
use tokio::io::{copy, AsyncRead};
use tokio::process::{Child, Command};

pub struct StartTokioChildProcess;

pub struct PrematureChildProcessExitError {
    pub exit_status: ExitStatus,
    pub output: String,
}

#[async_trait]
impl<Runtime> ChildProcessStarter<Runtime> for StartTokioChildProcess
where
    Runtime: HasChildProcessType<ChildProcess = Child>
        + HasFilePathType
        + CanSleep
        + CanPipeReaderToFile
        + CanReadFileAsString
        + CanRaiseError<IoError>
        + CanRaiseError<PrematureChildProcessExitError>,
    Runtime::FilePath: AsRef<Path>,
{
    async fn start_child_process(
        runtime: &Runtime,
        command_path: &Runtime::FilePath,
        command_args: &[&str],
        stdout_path: Option<&Runtime::FilePath>,
        stderr_path: Option<&Runtime::FilePath>,
    ) -> Result<Runtime::ChildProcess, Runtime::Error> {
        let mut child_process = Command::new(command_path.as_ref())
            .args(command_args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(Runtime::raise_error)?;

        if let Some(stdout_path) = stdout_path {
            if let Some(stdout) = child_process.stdout.take() {
                runtime.pipe_reader_to_file(stdout, stdout_path).await?;
            }
        }

        if let Some(stderr_path) = stderr_path {
            if let Some(stderr) = child_process.stderr.take() {
                runtime.pipe_reader_to_file(stderr, stderr_path).await?;
            }
        }

        // Wait for a while and check if the child process exited immediately.
        // If so, return error since we expect the child process to be running in the background.

        runtime.sleep(Duration::from_secs(1)).await;

        let status = child_process.try_wait().map_err(Runtime::raise_error)?;

        match status {
            None => Ok(child_process),
            Some(exit_status) => {
                let output = match stderr_path {
                    None => String::new(),
                    Some(stderr_path) => runtime.read_file_as_string(stderr_path).await?,
                };

                Err(Runtime::raise_error(PrematureChildProcessExitError {
                    exit_status,
                    output,
                }))
            }
        }
    }
}

#[async_trait]
pub trait CanPipeReaderToFile: HasFilePathType + HasErrorType {
    async fn pipe_reader_to_file(
        &self,
        reader: impl AsyncRead + Unpin + Send + Sync + 'static,
        write_file: &Self::FilePath,
    ) -> Result<(), Self::Error>;
}

#[async_trait]
impl<Runtime> CanPipeReaderToFile for Runtime
where
    Runtime: HasFilePathType + CanSpawnTask + CanRaiseError<IoError>,
    Runtime::FilePath: AsRef<Path>,
{
    async fn pipe_reader_to_file(
        &self,
        mut reader: impl AsyncRead + Unpin + Send + Sync + 'static,
        file_path: &Self::FilePath,
    ) -> Result<(), Self::Error> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
            .await
            .map_err(Runtime::raise_error)?;

        self.spawn_task(FutureTask::new(async move {
            let _ = copy(&mut reader, &mut file).await;
        }));

        Ok(())
    }
}
