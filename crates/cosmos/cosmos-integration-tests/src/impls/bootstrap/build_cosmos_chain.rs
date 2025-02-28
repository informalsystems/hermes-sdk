use core::time::Duration;

use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::HasDynamicGas;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_error::types::HermesError;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::{LevelDebug, LevelTrace};
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::traits::bootstrap::build_chain::{
    ChainBuilderWithNodeConfig, ChainBuilderWithNodeConfigComponent,
};
use crate::traits::bootstrap::cosmos_builder::HasCosmosBuilder;
use crate::traits::bootstrap::relayer_chain_config::CanBuildRelayerChainConfig;

const RETRY_START: u64 = 40;

pub struct BuildCosmosChainWithNodeConfig;

#[cgp_provider(ChainBuilderWithNodeConfigComponent)]
impl<Bootstrap> ChainBuilderWithNodeConfig<Bootstrap> for BuildCosmosChainWithNodeConfig
where
    Bootstrap: HasChainType<Chain = CosmosChain>
        + HasChainNodeConfigType
        + CanBuildRelayerChainConfig
        + HasCosmosBuilder
        + HasRuntime
        + HasDynamicGas
        + CanRaiseAsyncError<HermesError>,
    Bootstrap::Runtime: CanSleep,
    Bootstrap::Chain: CanQueryChainStatus
        + HasChainId
        + HasLogger<Logger: CanLog<LevelDebug> + CanLog<LevelTrace>>,
{
    async fn build_chain_with_node_config(
        bootstrap: &Bootstrap,
        chain_node_config: &Bootstrap::ChainNodeConfig,
        chain_genesis_config: &Bootstrap::ChainGenesisConfig,
        relayer_wallet: &CosmosTestWallet,
    ) -> Result<CosmosChain, Bootstrap::Error> {
        let relayer_chain_config = bootstrap
            .build_relayer_chain_config(chain_node_config, chain_genesis_config, relayer_wallet)
            .await?;

        let chain = bootstrap
            .cosmos_builder()
            .build_chain_with_config(relayer_chain_config.clone())
            .await
            .map_err(Bootstrap::raise_error)?;

        chain
            .logger()
            .log(
                &format!(
                    "Waiting for chain `{}` to reach height 5`",
                    chain.chain_id()
                ),
                &LevelDebug,
            )
            .await;

        // Wait for both RPC and gRPC servers to start before resuming bootstrapping
        for _ in 0..RETRY_START {
            if let Ok(status) = chain.query_chain_status().await {
                let current_height = status.height.revision_height();
                if current_height > 4 {
                    break;
                }
                chain
                    .logger()
                    .log(&format!("Current height `{current_height}`"), &LevelTrace)
                    .await;
            }

            bootstrap.runtime().sleep(Duration::from_millis(500)).await;
        }

        Ok(chain)
    }
}
