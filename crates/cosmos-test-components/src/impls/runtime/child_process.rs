use core::pin::Pin;
use std::io::Error as IoError;
use std::path::Path;
use std::process::Stdio;

use cgp_core::prelude::*;
use futures::Future;
use ibc_relayer_components::runtime::traits::task::Task;
use ibc_relayer_components_extra::runtime::traits::spawn::CanSpawnTask;
use tokio::fs::File;
use tokio::io::copy;
use tokio::process::{Child, Command};

use ibc_test_components::runtime::traits::child_process::ChildProcessStarter;
use ibc_test_components::runtime::traits::types::child_process::HasChildProcessType;
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;

pub struct StartTokioChildProcess;

#[async_trait]
impl<Runtime> ChildProcessStarter<Runtime> for StartTokioChildProcess
where
    Runtime:
        HasChildProcessType<ChildProcess = Child> + HasFilePathType + HasErrorType + CanSpawnTask,
    Runtime::FilePath: AsRef<Path>,
    Runtime::Error: From<IoError>,
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
            .spawn()?;

        if let Some(stdout_path) = stdout_path {
            if let Some(mut stdout) = child_process.stdout.take() {
                let mut stdout_file = File::create(&stdout_path).await?;

                runtime.spawn_task(FutureTask::new(async move {
                    let _ = copy(&mut stdout, &mut stdout_file).await;
                }));
            }
        }

        if let Some(stderr_path) = stderr_path {
            if let Some(mut stderr) = child_process.stderr.take() {
                let mut stderr_file = File::open(&stderr_path).await?;

                runtime.spawn_task(FutureTask::new(async move {
                    let _ = copy(&mut stderr, &mut stderr_file).await;
                }));
            }
        }

        Ok(child_process)
    }
}

pub struct FutureTask {
    pub future: Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>,
}

impl FutureTask {
    pub fn new(future: impl Future<Output = ()> + Send + Sync + 'static) -> Self {
        Self {
            future: Box::pin(future),
        }
    }
}

#[async_trait]
impl Task for FutureTask {
    async fn run(self) {
        self.future.await;
    }
}
