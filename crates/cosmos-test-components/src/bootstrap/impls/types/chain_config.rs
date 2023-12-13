use cgp_core::Async;

use crate::bootstrap::traits::types::chain_config::ProvideChainConfigType;
use crate::bootstrap::types::chain_config::CosmosChainConfig;

pub struct ProvideCosmosChainConfigType;

impl<Bootstrap> ProvideChainConfigType<Bootstrap> for ProvideCosmosChainConfigType
where
    Bootstrap: Async,
{
    type ChainConfig = CosmosChainConfig;
}
