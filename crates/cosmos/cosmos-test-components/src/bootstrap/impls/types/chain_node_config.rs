use hermes_prelude::*;

use crate::bootstrap::traits::{ChainNodeConfigTypeComponent, ProvideChainNodeConfigType};
use crate::bootstrap::types::CosmosChainNodeConfig;

pub struct ProvideCosmosChainNodeConfigType;

#[cgp_provider(ChainNodeConfigTypeComponent)]
impl<Bootstrap> ProvideChainNodeConfigType<Bootstrap> for ProvideCosmosChainNodeConfigType
where
    Bootstrap: Async,
{
    type ChainNodeConfig = CosmosChainNodeConfig;
}
