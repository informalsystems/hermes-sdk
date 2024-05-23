use cgp_core::CanRaiseError;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::CanStartChildProcess;
use hermes_runtime_components::traits::os::reserve_port::CanReserveTcpPort;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::bootstrap::traits::rollup_command_path::HasRollupCommandPath;
use crate::bootstrap::traits::start_rollup::RollupStarter;

pub struct StartSovereignRollup;

impl<Bootstrap, Runtime> RollupStarter<Bootstrap> for StartSovereignRollup
where
    Bootstrap: HasRuntime<Runtime = Runtime> + HasRollupCommandPath + CanRaiseError<Runtime::Error>,
    Runtime: HasFilePathType + CanStartChildProcess + CanReserveTcpPort,
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

        let runtime = bootstrap.runtime();

        let metrics_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

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
                    "--metrics",
                    &metrics_port.to_string(),
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
