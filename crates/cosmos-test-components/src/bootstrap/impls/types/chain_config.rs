use cgp_core::Async;

use crate::bootstrap::impls::initializers::update_chain_config::CosmosChainConfig;
use crate::bootstrap::traits::types::chain_config::ProvideChainConfigType;

pub struct ProvideCosmosChainConfigType;

impl<Bootstrap> ProvideChainConfigType<Bootstrap> for ProvideCosmosChainConfigType
where
    Bootstrap: Async,
{
    type ChainConfig = CosmosChainConfig;
}
