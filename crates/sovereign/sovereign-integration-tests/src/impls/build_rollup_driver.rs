use alloc::collections::BTreeMap;
use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_client_components::sovereign::traits::chain::rollup::HasRollupType;
use hermes_sovereign_cosmos_relayer::contexts::sovereign_rollup::SovereignRollup;
use hermes_sovereign_test_components::bootstrap::traits::build_rollup_driver::RollupDriverBuilder;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_node_config::HasRollupNodeConfigType;
use hermes_sovereign_test_components::types::rollup_genesis_config::SovereignGenesisConfig;
use hermes_sovereign_test_components::types::rollup_node_config::SovereignRollupNodeConfig;
use hermes_sovereign_test_components::types::wallet::SovereignWallet;
use jsonrpsee::core::ClientError;
use jsonrpsee::http_client::HttpClientBuilder;
use tokio::process::Child;

use crate::contexts::rollup_driver::SovereignRollupDriver;

pub struct BuildSovereignRollupDriver;

impl<Bootstrap> RollupDriverBuilder<Bootstrap> for BuildSovereignRollupDriver
where
    Bootstrap: HasRuntime<Runtime = HermesRuntime>
        + HasRollupType<Rollup = SovereignRollup>
        + HasRollupDriverType<RollupDriver = SovereignRollupDriver>
        + HasRollupNodeConfigType<RollupNodeConfig = SovereignRollupNodeConfig>
        + HasRollupGenesisConfigType<RollupGenesisConfig = SovereignGenesisConfig>
        + CanRaiseError<ClientError>,
{
    async fn build_rollup_driver(
        bootstrap: &Bootstrap,
        node_config: SovereignRollupNodeConfig,
        genesis_config: SovereignGenesisConfig,
        wallets: BTreeMap<String, SovereignWallet>,
        rollup_process: Child,
    ) -> Result<SovereignRollupDriver, Bootstrap::Error> {
        let rpc_config = &node_config.runner.rpc_config;
        let rpc_url = format!("http://{}:{}", rpc_config.bind_host, rpc_config.bind_port);

        let rpc_client = HttpClientBuilder::default()
            .build(rpc_url)
            .map_err(Bootstrap::raise_error)?;

        let rollup = SovereignRollup {
            runtime: bootstrap.runtime().clone(),
            rpc_client,
        };

        Ok(SovereignRollupDriver {
            rollup,
            node_config,
            genesis_config,
            wallets,
            rollup_process,
        })
    }
}
