use crate::runtime::traits::exec_command::{CanExecCommandWithEnvs, CommandExecutor, ExecOutput};

pub struct ExecCommandWithNoEnv;

impl<Runtime> CommandExecutor<Runtime> for ExecCommandWithNoEnv
where
    Runtime: CanExecCommandWithEnvs,
{
    async fn exec_command(
        runtime: &Runtime,
        command_path: &Runtime::FilePath,
        args: &[&str],
    ) -> Result<ExecOutput, Runtime::Error> {
        runtime
            .exec_command_with_envs(command_path, args, &[])
            .await
    }
}
