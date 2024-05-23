use alloc::collections::BTreeMap;

use cgp_core::prelude::*;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::HasRollupType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};

use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use crate::bootstrap::traits::types::rollup_node_config::HasRollupNodeConfigType;

#[derive_component(RollupDriverBuilderComponent, RollupDriverBuilder<Bootstrap>)]
#[async_trait]
pub trait CanBuildRollupDriver:
    HasRuntimeType
    + HasRollupType
    + HasRollupDriverType
    + HasRollupNodeConfigType
    + HasRollupGenesisConfigType
    + HasErrorType
where
    Self::Runtime: HasChildProcessType,
    Self::Rollup: HasWalletType,
{
    async fn build_rollup_driver(
        &self,
        rollup_node_config: Self::RollupNodeConfig,
        rollup_genesis_config: Self::RollupGenesisConfig,
        rollup_wallets: BTreeMap<String, WalletOf<Self::Rollup>>,
        rollup_process: ChildProcessOf<Self::Runtime>,
    ) -> Result<Self::RollupDriver, Self::Error>;
}
