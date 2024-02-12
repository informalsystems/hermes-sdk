use alloc::collections::BTreeMap;
use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::chain_driver::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcessOf, HasChildProcessType,
};

use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;
use crate::bootstrap::traits::types::rollup_node_config::HasRollupNodeConfigType;

#[derive_component(RollupDriverBuilderComponent, RollupDriverBuilder<Bootstrap>)]
#[async_trait]
pub trait CanBuildRollupDriver:
    HasRuntimeType
    + HasRollupDriverType
    + HasRollupNodeConfigType
    + HasRollupGenesisConfigType
    + HasErrorType
where
    Self::Runtime: HasChildProcessType,
    Self::RollupDriver: HasWalletType,
{
    async fn build_rollup_driver(
        &self,
        rollup_node_config: Self::RollupNodeConfig,
        rollup_genesis_config: Self::RollupGenesisConfig,
        rollup_wallets: BTreeMap<String, WalletOf<Self::RollupDriver>>,
        rollup_process: ChildProcessOf<Self::Runtime>,
    ) -> Result<Self::RollupDriver, Self::Error>;
}
