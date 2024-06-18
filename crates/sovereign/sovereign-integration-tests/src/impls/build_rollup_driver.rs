use alloc::collections::BTreeMap;
use core::time::Duration;

use cgp_core::CanRaiseError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::HasRollupType;
use hermes_sovereign_relayer::contexts::sovereign_rollup::SovereignRollup;
use hermes_sovereign_test_components::bootstrap::traits::build_rollup_driver::RollupDriverBuilder;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_node_config::HasRollupNodeConfigType;
use hermes_sovereign_test_components::types::rollup_genesis_config::SovereignGenesisConfig;
use hermes_sovereign_test_components::types::rollup_node_config::SovereignRollupNodeConfig;
use hermes_sovereign_test_components::types::wallet::SovereignWallet;
use jsonrpsee::core::ClientError;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use tokio::process::Child;
use tokio::time::sleep;

use crate::contexts::rollup_driver::SovereignRollupDriver;

pub struct BuildSovereignRollupDriver;

impl<Bootstrap> RollupDriverBuilder<Bootstrap> for BuildSovereignRollupDriver
where
    Bootstrap: HasRuntime<Runtime = HermesRuntime>
        + HasRollupType<Rollup = SovereignRollup>
        + HasRollupDriverType<RollupDriver = SovereignRollupDriver>
        + HasRollupNodeConfigType<RollupNodeConfig = SovereignRollupNodeConfig>
        + HasRollupGenesisConfigType<RollupGenesisConfig = SovereignGenesisConfig>
        + CanRaiseError<ClientError>
        + CanRaiseError<&'static str>,
{
    async fn build_rollup_driver(
        bootstrap: &Bootstrap,
        node_config: SovereignRollupNodeConfig,
        genesis_config: SovereignGenesisConfig,
        wallets: BTreeMap<String, SovereignWallet>,
        rollup_process: Child,
    ) -> Result<SovereignRollupDriver, Bootstrap::Error> {
        let rpc_config = &node_config.runner.rpc_config;
        let bind_host = &rpc_config.bind_host;
        let bind_port = rpc_config.bind_port;

        let rpc_client = HttpClientBuilder::default()
            .build(format!("http://{}:{}", bind_host, bind_port))
            .map_err(Bootstrap::raise_error)?;

        let subscription_client = build_subscription_client(bind_host, bind_port)
            .await
            .map_err(Bootstrap::raise_error)?;

        let relayer_wallet = wallets
            .get("relayer")
            .ok_or_else(|| Bootstrap::raise_error("expect relayer wallet"))?;

        let rollup = SovereignRollup::new(
            bootstrap.runtime().clone(),
            relayer_wallet.signing_key.clone(),
            rpc_client,
            subscription_client,
        );

        Ok(SovereignRollupDriver {
            rollup,
            node_config,
            genesis_config,
            wallets,
            rollup_process,
        })
    }
}

async fn build_subscription_client(
    bind_host: &String,
    bind_port: u16,
) -> Result<WsClient, ClientError> {
    let build_client =
        || WsClientBuilder::default().build(format!("ws://{}:{}", bind_host, bind_port));

    // The WebSocket server may not start in time during test.
    // In such case, we will wait for at most 10s for the server to start
    for _ in 0..10 {
        let result = build_client().await;

        if let Ok(client) = result {
            return Ok(client);
        }

        sleep(Duration::from_secs(1)).await;
    }

    build_client().await
}
