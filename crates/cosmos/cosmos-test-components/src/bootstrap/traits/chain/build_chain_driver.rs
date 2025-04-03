use alloc::collections::BTreeMap;

use cgp::prelude::*;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;

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
        chain_process: ChildProcessOf<Self::Runtime>,
    ) -> Result<Self::ChainDriver, Self::Error>;
}
