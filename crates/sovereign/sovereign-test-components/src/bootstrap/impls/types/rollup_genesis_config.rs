use cgp_core::Async;

use crate::bootstrap::traits::types::rollup_genesis_config::ProvideRollupGenesisConfigType;
use crate::types::rollup_genesis_config::SovereignGenesisConfig;

pub struct ProvideSovereignGenesisConfig;

impl<Bootstrap> ProvideRollupGenesisConfigType<Bootstrap> for ProvideSovereignGenesisConfig
where
    Bootstrap: Async,
{
    type RollupGenesisConfig = SovereignGenesisConfig;
}
