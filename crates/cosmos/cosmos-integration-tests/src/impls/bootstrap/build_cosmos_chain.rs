use cgp::prelude::*;
use hermes_cosmos_chain_components::traits::CanQueryGrpcServiceStatus;
use hermes_cosmos_relayer::contexts::CosmosChain;
use hermes_cosmos_test_components::bootstrap::traits::{HasChainNodeConfigType, HasDynamicGas};
use hermes_cosmos_test_components::chain::types::CosmosTestWallet;
use hermes_error::types::HermesError;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::{LevelDebug, LevelTrace, LevelWarn};
use hermes_relayer_components::chain::traits::{CanQueryChainStatus, HasChainId, HasPollInterval};
use hermes_runtime_components::traits::{CanSleep, HasRuntime};
use hermes_test_components::chain_driver::traits::HasChainType;

use crate::traits::{
    CanBuildRelayerChainConfig, ChainBuilderWithNodeConfig, ChainBuilderWithNodeConfigComponent,
    HasCosmosBuilder,
};

const RETRY_COUNT: u64 = 100;

#[cgp_new_provider(ChainBuilderWithNodeConfigComponent)]
impl<Bootstrap> ChainBuilderWithNodeConfig<Bootstrap> for BuildCosmosChainWithNodeConfig
where
    Bootstrap: HasChainType<Chain = CosmosChain>
        + HasChainNodeConfigType
        + CanBuildRelayerChainConfig
        + HasCosmosBuilder
        + HasRuntime
        + HasDynamicGas
        + CanLog<LevelWarn>
        + CanLog<LevelDebug>
        + CanLog<LevelTrace>
        + CanRaiseAsyncError<HermesError>,
    Bootstrap::Runtime: CanSleep,
    Bootstrap::Chain:
        CanQueryChainStatus + CanQueryGrpcServiceStatus + HasPollInterval + HasChainId,
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
            .log(
                &format!(
                    "waiting for chain `{}` RPC and GRPC services to become ready`",
                    chain.chain_id()
                ),
                &LevelDebug,
            )
            .await;

        // Wait for both RPC and gRPC servers to start before resuming bootstrapping
        for i in 0..RETRY_COUNT {
            let rpc_server_is_ready = chain.query_chain_status().await.is_ok();

            let grpc_server_is_ready = chain
                .query_grpc_service_status_is_ready()
                .await
                .unwrap_or(false);

            if rpc_server_is_ready && grpc_server_is_ready {
                chain
                    .log(
                        &format!(
                            "RPC and GRPC services of chain `{}`  are now ready",
                            chain.chain_id()
                        ),
                        &LevelDebug,
                    )
                    .await;

                break;
            } else if i + 1 == RETRY_COUNT {
                chain
                    .log(
                        &format!(
                            "RPC and GRPC services of chain `{}`  are still not ready after {} tries. Will continue with possible failure",
                            chain.chain_id(),
                            RETRY_COUNT,
                        ),
                        &LevelWarn,
                    )
                    .await;
            }

            bootstrap.runtime().sleep(chain.poll_interval()).await;
        }

        Ok(chain)
    }
}
