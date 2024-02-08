use cgp_core::prelude::*;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;

#[derive_component(RollupBootstrapperComponent, RollupBootstrapper<Bootstrap>)]
#[async_trait]
pub trait CanBootstrapRollup:
    HasChainDriverType + HasBridgeDriverType + HasRollupDriverType + HasErrorType
{
    async fn bootstrap_rollup(
        &self,
        chain_driver: &Self::ChainDriver,
        bridge_driver: &Self::BridgeDriver,
        rollup_id: &str,
    ) -> Result<Self::RollupDriver, Self::Error>;
}
