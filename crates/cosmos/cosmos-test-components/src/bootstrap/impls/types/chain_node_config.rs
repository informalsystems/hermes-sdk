use cgp::prelude::*;

use crate::bootstrap::traits::types::chain_node_config::{
    ChainNodeConfigTypeComponent, ProvideChainNodeConfigType,
};
use crate::bootstrap::types::chain_node_config::CosmosChainNodeConfig;

pub struct ProvideCosmosChainNodeConfigType;

#[cgp_provider(ChainNodeConfigTypeComponent)]
impl<Bootstrap> ProvideChainNodeConfigType<Bootstrap> for ProvideCosmosChainNodeConfigType
where
    Bootstrap: Async,
{
    type ChainNodeConfig = CosmosChainNodeConfig;
}
