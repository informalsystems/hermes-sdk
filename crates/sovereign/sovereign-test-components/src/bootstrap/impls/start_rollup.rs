use cgp_core::CanRaiseError;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::child_process::CanStartChildProcess;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

use crate::bootstrap::traits::rollup_command_path::HasRollupCommandPath;
use crate::bootstrap::traits::start_rollup::RollupStarter;

pub struct StartSovereignRollup;

impl<Bootstrap, Runtime> RollupStarter<Bootstrap> for StartSovereignRollup
where
    Bootstrap: HasRuntime<Runtime = Runtime> + HasRollupCommandPath + CanRaiseError<Runtime::Error>,
    Runtime: HasFilePathType + CanStartChildProcess,
{
    async fn start_rollup(
        bootstrap: &Bootstrap,
        rollup_home_dir: &Runtime::FilePath,
    ) -> Result<Runtime::ChildProcess, Bootstrap::Error> {
        let rollup_node_config_path = Runtime::join_file_path(
            rollup_home_dir,
            &Runtime::file_path_from_string("config.toml"),
        );

        let rollup_genesis_path =
            Runtime::join_file_path(rollup_home_dir, &Runtime::file_path_from_string("genesis"));

        let rollup_chain_state_path = Runtime::join_file_path(
            rollup_home_dir,
            &Runtime::file_path_from_string("genesis/chain_state.json"),
        );

        let stdout_path = Runtime::join_file_path(
            rollup_home_dir,
            &Runtime::file_path_from_string("stdout.log"),
        );

        let stderr_path = Runtime::join_file_path(
            rollup_home_dir,
            &Runtime::file_path_from_string("stderr.log"),
        );

        let child = bootstrap
            .runtime()
            .start_child_process(
                bootstrap.rollup_command_path(),
                &[
                    "--rollup-config-path",
                    &Runtime::file_path_to_string(&rollup_node_config_path),
                    "--genesis-paths",
                    &Runtime::file_path_to_string(&rollup_genesis_path),
                    "--kernel-genesis-paths",
                    &Runtime::file_path_to_string(&rollup_chain_state_path),
                ],
                &[("RUST_BACKTRACE", "full")],
                Some(&stdout_path),
                Some(&stderr_path),
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(child)
    }
}
