use alloc::collections::BTreeMap;

use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::chain_driver::traits::types::wallet::{HasWalletType, Wallet};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcessOf, HasChildProcessType,
};

use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;

#[derive_component(ChainDriverBuilderComponent, ChainDriverBuilder<Bootstrap>)]
#[async_trait]
pub trait CanBuildChainDriver:
    HasRuntimeType
    + HasChainDriverType
    + HasChainGenesisConfigType
    + HasChainNodeConfigType
    + HasErrorType
where
    Self::Runtime: HasChildProcessType,
    Self::ChainDriver: HasWalletType,
{
    async fn build_chain_driver(
        &self,
        genesis_config: Self::ChainGenesisConfig,
        chain_node_config: Self::ChainNodeConfig,
        wallets: BTreeMap<String, Wallet<Self::ChainDriver>>,
        chain_process: ChildProcessOf<Self::Runtime>,
    ) -> Result<Self::ChainDriver, Self::Error>;
}
