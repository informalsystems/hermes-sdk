use cgp_core::prelude::Async;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_test_components::chain_driver::traits::types::chain::{HasChainType, ProvideChainType};
use hermes_test_components::driver::traits::types::chain_driver::ProvideChainDriverType;

use crate::contexts::chain_driver::CosmosChainDriver;

pub struct ProvideCosmosBootstrapChainTypes;

impl<Bootstrap> ProvideChainType<Bootstrap> for ProvideCosmosBootstrapChainTypes
where
    Bootstrap: Async,
{
    type Chain = CosmosChain;
}

impl<Bootstrap> ProvideChainDriverType<Bootstrap> for ProvideCosmosBootstrapChainTypes
where
    Bootstrap: HasChainType<Chain = CosmosChain>,
{
    type ChainDriver = CosmosChainDriver;
}
