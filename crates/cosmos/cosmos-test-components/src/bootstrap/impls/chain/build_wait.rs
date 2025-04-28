use core::marker::PhantomData;
use std::collections::BTreeMap;

use cgp::prelude::*;
use hermes_core::runtime_components::traits::{
    ChildProcessOf, HasChildProcessType, HasRuntimeType,
};
use hermes_core::test_components::chain::traits::{HasWalletType, Wallet};
use hermes_core::test_components::chain_driver::traits::{CanWaitChainStartup, HasChainType};
use hermes_core::test_components::driver::traits::HasChainDriverType;

use crate::bootstrap::traits::{
    ChainDriverBuilder, ChainDriverBuilderComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType,
};

pub struct BuildAndWaitChainDriver<InBuilder>(pub PhantomData<InBuilder>);

#[cgp_provider(ChainDriverBuilderComponent)]
impl<Bootstrap, ChainDriver, InBuilder> ChainDriverBuilder<Bootstrap>
    for BuildAndWaitChainDriver<InBuilder>
where
    Bootstrap: HasRuntimeType<Runtime: HasChildProcessType>
        + HasChainType<Chain: HasWalletType>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasChainGenesisConfigType
        + HasChainNodeConfigType
        + CanRaiseAsyncError<ChainDriver::Error>,
    InBuilder: ChainDriverBuilder<Bootstrap>,
    ChainDriver: CanWaitChainStartup,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: Bootstrap::ChainGenesisConfig,
        chain_node_config: Bootstrap::ChainNodeConfig,
        wallets: BTreeMap<String, Wallet<Bootstrap::Chain>>,
        chain_process: Vec<ChildProcessOf<Bootstrap::Runtime>>,
    ) -> Result<ChainDriver, Bootstrap::Error> {
        let chain_driver = InBuilder::build_chain_driver(
            bootstrap,
            genesis_config,
            chain_node_config,
            wallets,
            chain_process,
        )
        .await?;

        chain_driver
            .wait_chain_startup()
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(chain_driver)
    }
}
