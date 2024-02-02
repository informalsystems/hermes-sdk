use cgp_core::prelude::*;

use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;

#[derive_component(RollupGenesisGeneratorComponent, RollupGenesisGenerator<Bootstrap>)]
#[async_trait]
pub trait CanGenerateRollupGenesis: HasRollupGenesisConfigType + HasErrorType {
    async fn generate_rollup_genesis(&self) -> Result<Self::RollupGenesisConfig, Self::Error>;
}
