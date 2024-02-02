use cgp_core::prelude::*;
use hermes_test_components::chain_driver::traits::types::address::{AddressOf, HasAddressType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;

#[derive_component(RollupGenesisGeneratorComponent, RollupGenesisGenerator<Bootstrap>)]
#[async_trait]
pub trait CanGenerateRollupGenesis:
    HasChainDriverType + HasRollupGenesisConfigType + HasErrorType
where
    Self::ChainDriver: HasAddressType,
{
    async fn generate_rollup_genesis(
        &self,
        sequencer_da_address: &AddressOf<Self::ChainDriver>,
    ) -> Result<Self::RollupGenesisConfig, Self::Error>;
}
