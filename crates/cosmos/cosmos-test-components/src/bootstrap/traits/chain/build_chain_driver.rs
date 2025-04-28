use alloc::collections::BTreeMap;

use hermes_core::runtime_components::traits::{
    ChildProcessOf, HasChildProcessType, HasRuntimeType,
};
use hermes_core::test_components::chain::traits::{HasWalletType, Wallet};
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_core::test_components::driver::traits::HasChainDriverType;
use hermes_prelude::*;

use crate::bootstrap::traits::{HasChainGenesisConfigType, HasChainNodeConfigType};

#[cgp_component {
  provider: ChainDriverBuilder,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanBuildChainDriver:
    HasRuntimeType<Runtime: HasChildProcessType>
    + HasChainDriverType
    + HasChainType<Chain: HasWalletType>
    + HasChainGenesisConfigType
    + HasChainNodeConfigType
    + HasAsyncErrorType
{
    async fn build_chain_driver(
        &self,
        genesis_config: Self::ChainGenesisConfig,
        chain_node_config: Self::ChainNodeConfig,
        wallets: BTreeMap<String, Wallet<Self::Chain>>,
        chain_processes: Vec<ChildProcessOf<Self::Runtime>>,
    ) -> Result<Self::ChainDriver, Self::Error>;
}
