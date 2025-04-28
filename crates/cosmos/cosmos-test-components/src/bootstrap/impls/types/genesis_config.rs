use hermes_prelude::*;

use crate::bootstrap::traits::{ChainGenesisConfigTypeComponent, ProvideChainGenesisConfigType};
use crate::bootstrap::types::CosmosGenesisConfig;

pub struct ProvideCosmosGenesisConfigType;

#[cgp_provider(ChainGenesisConfigTypeComponent)]
impl<Bootstrap> ProvideChainGenesisConfigType<Bootstrap> for ProvideCosmosGenesisConfigType
where
    Bootstrap: Async,
{
    type ChainGenesisConfig = CosmosGenesisConfig;
}
