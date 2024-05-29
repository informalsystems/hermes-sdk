use core::fmt::Display;

use cgp_core::CanRaiseError;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use hermes_celestia_test_components::bridge_driver::traits::bridge_auth_token::HasBridgeAuthToken;
use hermes_celestia_test_components::bridge_driver::traits::bridge_rpc_port::HasBridgeRpcPort;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::os::reserve_port::CanReserveTcpPort;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::init_rollup_node_config::RollupNodeConfigInitializer;
use crate::bootstrap::traits::types::rollup_node_config::HasRollupNodeConfigType;
use crate::types::rollup_node_config::{
    SovereignAxumConfig, SovereignDaConfig, SovereignProverConfig, SovereignRollupNodeConfig,
    SovereignRpcConfig, SovereignRunnerConfig, SovereignStorageConfig,
};

pub struct InitSovereignRollupNodeConfig;

impl<Bootstrap, Chain, BridgeDriver, Runtime> RollupNodeConfigInitializer<Bootstrap>
    for InitSovereignRollupNodeConfig
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasBridgeDriverType<BridgeDriver = BridgeDriver>
        + HasRollupNodeConfigType
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<toml::ser::Error>,
    Runtime: CanReserveTcpPort + CanWriteStringToFile,
    Chain: HasAddressType,
    BridgeDriver: HasBridgeRpcPort + HasBridgeAuthToken,
    BridgeDriver::BridgeAuthToken: Display,
    Bootstrap::RollupNodeConfig: From<SovereignRollupNodeConfig>,
{
    async fn init_rollup_node_config(
        bootstrap: &Bootstrap,
        rollup_home_dir: &Runtime::FilePath,
        bridge_driver: &BridgeDriver,
        sequencer_da_address: &Chain::Address,
    ) -> Result<Bootstrap::RollupNodeConfig, Bootstrap::Error> {
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

        let rollup_axum_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

        let rollup_node_config = SovereignRollupNodeConfig {
            da: SovereignDaConfig {
                celestia_rpc_auth_token: auth_token.to_string(),
                celestia_rpc_address: format!("http://127.0.0.1:{bridge_rpc_port}"),
                max_celestia_response_body_size: 104_857_600,
                celestia_rpc_timeout_seconds: 60,
                own_celestia_address: sequencer_da_address.to_string(),
            },
            storage: SovereignStorageConfig {
                path: Runtime::file_path_to_string(&data_path),
            },
            runner: SovereignRunnerConfig {
                genesis_height: 1,
                rpc_config: SovereignRpcConfig {
                    bind_host: "127.0.0.1".into(),
                    bind_port: rollup_rpc_port,
                },
                axum_config: SovereignAxumConfig {
                    bind_host: "127.0.0.1".into(),
                    bind_port: rollup_axum_port,
                },
                da_polling_interval_ms: 10000,
            },
            proof_manager: SovereignProverConfig {
                aggregated_proof_block_jump: 1,
            },
        };

        let rollup_node_config_str =
            toml::to_string_pretty(&rollup_node_config).map_err(Bootstrap::raise_error)?;

        runtime
            .write_string_to_file(&config_path, &rollup_node_config_str)
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(rollup_node_config.into())
    }
}
