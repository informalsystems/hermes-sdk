use cgp::prelude::*;
use hermes_relayer_components::chain::traits::HasChainId;
use hermes_runtime_components::traits::{
    CanGenerateRandom, HasChildProcessType, HasFilePathType, HasRuntime,
};
use hermes_test_components::chain_driver::traits::{HasChain, HasChainType};
use hermes_test_components::driver::traits::HasChainDriverType;

use crate::bootstrap::traits::bootstrap_bridge::{BridgeBootstrapper, BridgeBootstrapperComponent};
use crate::bootstrap::traits::bridge_auth_token::CanGenerateBridgeAuthToken;
use crate::bootstrap::traits::bridge_store_dir::HasBridgeStoreDir;
use crate::bootstrap::traits::build_bridge_driver::CanBuildBridgeDriver;
use crate::bootstrap::traits::import_bridge_key::CanImportBridgeKey;
use crate::bootstrap::traits::init_bridge_config::CanInitBridgeConfig;
use crate::bootstrap::traits::init_bridge_data::CanInitBridgeData;
use crate::bootstrap::traits::start_bridge::CanStartBridge;
use crate::bridge_driver::traits::bridge_auth_token::HasBridgeAuthTokenType;

pub struct BootstrapCelestiaBridge;

#[cgp_provider(BridgeBootstrapperComponent)]
impl<Bootstrap, Chain, ChainDriver, Runtime> BridgeBootstrapper<Bootstrap>
    for BootstrapCelestiaBridge
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasRuntime<Runtime = Runtime>
        + HasBridgeStoreDir
        + CanInitBridgeData
        + CanImportBridgeKey
        + CanGenerateBridgeAuthToken
        + CanInitBridgeConfig
        + CanStartBridge
        + CanBuildBridgeDriver,
    ChainDriver: HasChain<Chain = Chain> + HasRuntime<Runtime = Runtime>,
    Chain: HasChainId,
    Runtime: HasFilePathType + HasChildProcessType + CanGenerateRandom<u32>,
    Bootstrap::BridgeDriver: HasBridgeAuthTokenType,
{
    async fn bootstrap_bridge(
        bootstrap: &Bootstrap,
        chain_driver: &ChainDriver,
    ) -> Result<Bootstrap::BridgeDriver, Bootstrap::Error> {
        let chain_id = chain_driver.chain().chain_id();
        let bridge_store_dir = bootstrap.bridge_store_dir();
        let bridge_postfix = bootstrap.runtime().generate_random().await;

        let bridge_home_dir = Runtime::join_file_path(
            bridge_store_dir,
            &Runtime::file_path_from_string(&format!("{chain_id}-{bridge_postfix}")),
        );

        bootstrap
            .init_bridge_data(&bridge_home_dir, chain_id)
            .await?;

        let bridge_config = bootstrap
            .init_bridge_config(&bridge_home_dir, chain_driver)
            .await?;

        bootstrap
            .import_bridge_key(&bridge_home_dir, chain_driver)
            .await?;

        let bridge_auth_token = bootstrap
            .generate_bridge_auth_token(&bridge_home_dir, chain_id)
            .await?;

        let bridge_process = bootstrap
            .start_bridge(&bridge_home_dir, &bridge_config, chain_driver)
            .await?;

        let bridge_driver = bootstrap
            .build_bridge_driver(bridge_config, bridge_auth_token, bridge_process)
            .await?;

        Ok(bridge_driver)
    }
}
