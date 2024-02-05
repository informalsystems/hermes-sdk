use core::fmt::Display;

use cgp_core::CanRaiseError;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use hermes_celestia_test_components::bridge_driver::traits::bridge_auth_token::HasBridgeAuthToken;
use hermes_celestia_test_components::bridge_driver::traits::bridge_rpc_port::HasBridgeRpcPort;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::reserve_port::CanReserveTcpPort;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;

use crate::bootstrap::traits::init_rollup_config::RollupConfigInitializer;
use crate::bootstrap::traits::types::rollup_config::HasRollupConfigType;
use crate::types::rollup_config::{
    SovereignDaConfig, SovereignProverConfig, SovereignRollupConfig, SovereignRpcConfig,
    SovereignRunnerConfig, SovereignStorageConfig,
};

pub struct InitSovereignRollupConfig;

impl<Bootstrap, BridgeDriver, Runtime> RollupConfigInitializer<Bootstrap>
    for InitSovereignRollupConfig
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasBridgeDriverType<BridgeDriver = BridgeDriver>
        + HasRollupConfigType
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<toml::ser::Error>,
    Runtime: CanReserveTcpPort + CanWriteStringToFile,
    BridgeDriver: HasBridgeRpcPort + HasBridgeAuthToken,
    BridgeDriver::BridgeAuthToken: Display,
    Bootstrap::RollupConfig: From<SovereignRollupConfig>,
{
    async fn init_rollup_config(
        bootstrap: &Bootstrap,
        rollup_home_dir: &Runtime::FilePath,
        bridge_driver: &BridgeDriver,
    ) -> Result<Bootstrap::RollupConfig, Bootstrap::Error> {
        let runtime = bootstrap.runtime();

        let bridge_rpc_port = bridge_driver.bridge_rpc_port();
        let auth_token = bridge_driver.bridge_auth_token();

        let config_path = Runtime::join_file_path(
            rollup_home_dir,
            &Runtime::file_path_from_string("config.toml"),
        );

        let data_path =
            Runtime::join_file_path(rollup_home_dir, &Runtime::file_path_from_string("data"));

        let rollup_rpc_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

        let rollup_config = SovereignRollupConfig {
            da: SovereignDaConfig {
                celestia_rpc_auth_token: auth_token.to_string(),
                celestia_rpc_address: format!("http://127.0.0.1:{bridge_rpc_port}"),
                max_celestia_response_body_size: 104_857_600,
                celestia_rpc_timeout_seconds: 60,
            },
            storage: SovereignStorageConfig {
                path: Runtime::file_path_to_string(&data_path),
            },
            runner: SovereignRunnerConfig {
                start_height: 1,
                rpc_config: SovereignRpcConfig {
                    bind_host: "127.0.0.1".into(),
                    bind_port: rollup_rpc_port,
                },
            },
            prover_service: SovereignProverConfig {
                aggregated_proof_block_jump: 1,
            },
        };

        let rollup_config_str =
            toml::to_string_pretty(&rollup_config).map_err(Bootstrap::raise_error)?;

        runtime
            .write_string_to_file(&config_path, &rollup_config_str)
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(rollup_config.into())
    }
}
