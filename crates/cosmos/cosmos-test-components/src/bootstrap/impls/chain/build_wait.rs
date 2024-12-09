use core::marker::PhantomData;
use std::collections::BTreeMap;

use cgp::prelude::CanRaiseError;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::chain_driver::traits::wait::CanWaitChainStartup;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilder;
use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;

pub struct BuildAndWaitChainDriver<InBuilder>(pub PhantomData<InBuilder>);

impl<Bootstrap, ChainDriver, InBuilder> ChainDriverBuilder<Bootstrap>
    for BuildAndWaitChainDriver<InBuilder>
where
    Bootstrap: HasRuntimeType<Runtime: HasChildProcessType>
        + HasChainType<Chain: HasWalletType>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasChainGenesisConfigType
        + HasChainNodeConfigType
        + CanRaiseError<ChainDriver::Error>,
    InBuilder: ChainDriverBuilder<Bootstrap>,
    ChainDriver: CanWaitChainStartup,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: Bootstrap::ChainGenesisConfig,
        chain_node_config: Bootstrap::ChainNodeConfig,
        wallets: BTreeMap<String, Wallet<Bootstrap::Chain>>,
        chain_process: ChildProcessOf<Bootstrap::Runtime>,
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
