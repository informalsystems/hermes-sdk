use core::time::Duration;

use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::HasDynamicGas;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_error::types::HermesError;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::traits::bootstrap::build_chain::{
    ChainBuilderWithNodeConfig, ChainBuilderWithNodeConfigComponent,
};
use crate::traits::bootstrap::cosmos_builder::HasCosmosBuilder;
use crate::traits::bootstrap::relayer_chain_config::CanBuildRelayerChainConfig;

const RETRY_START: u64 = 20;

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
    Bootstrap::Chain: CanQueryChainStatus,
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

        // TODO: Have a more reliable way to wait for the bootstrapped full node to
        // start up. If we don't wait, the building of the chain would fail during
        // the spawning of `ChainHandle`, as the v1 relayer tries to perform health
        // check on building.
        bootstrap.runtime().sleep(Duration::from_secs(6)).await;

        let chain = bootstrap
            .cosmos_builder()
            .build_chain_with_config(relayer_chain_config.clone())
            .await
            .map_err(Bootstrap::raise_error)?;

        for _ in 0..RETRY_START {
            if chain.query_chain_status().await.is_ok() {
                break;
            }

            bootstrap.runtime().sleep(Duration::from_millis(500)).await;
        }

        Ok(chain)
    }
}
