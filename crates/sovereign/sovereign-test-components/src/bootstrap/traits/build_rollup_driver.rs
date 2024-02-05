use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcessOf, HasChildProcessType,
};

use crate::bootstrap::traits::types::rollup_config::HasRollupConfigType;
use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;

#[derive_component(RollupDriverBuilderComponent, RollupDriverBuilder<Bootstrap>)]
#[async_trait]
pub trait CanBuildRollupDriver:
    HasRuntimeType
    + HasRollupDriverType
    + HasRollupConfigType
    + HasRollupGenesisConfigType
    + HasErrorType
where
    Self::Runtime: HasChildProcessType,
{
    async fn build_rollup_driver(
        &self,
        rollup_config: Self::RollupConfig,
        rollup_genesis_config: Self::RollupGenesisConfig,
        rollup_process: ChildProcessOf<Self::Runtime>,
    ) -> Result<Self::RollupDriver, Self::Error>;
}
