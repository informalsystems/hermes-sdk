use alloc::collections::BTreeMap;

use cgp_core::prelude::*;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

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
    Self::Chain: HasWalletType,
{
    async fn build_chain_driver(
        &self,
        genesis_config: Self::ChainGenesisConfig,
        chain_node_config: Self::ChainNodeConfig,
        wallets: BTreeMap<String, Wallet<Self::Chain>>,
        chain_process: ChildProcessOf<Self::Runtime>,
    ) -> Result<Self::ChainDriver, Self::Error>;
}
