use cgp_core::Async;

use crate::bootstrap::traits::types::chain_node_config::ProvideChainNodeConfigType;
use crate::bootstrap::types::chain_node_config::CosmosChainNodeConfig;

pub struct ProvideCosmosChainNodeConfigType;

impl<Bootstrap> ProvideChainNodeConfigType<Bootstrap> for ProvideCosmosChainNodeConfigType
where
    Bootstrap: Async,
{
    type ChainNodeConfig = CosmosChainNodeConfig;
}
