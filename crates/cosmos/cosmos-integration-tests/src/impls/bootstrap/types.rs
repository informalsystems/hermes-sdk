use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainTypeComponent, HasChainType, ProvideChainType,
};
use hermes_test_components::driver::traits::types::chain_driver::{
    ChainDriverTypeComponent, ProvideChainDriverType,
};

use crate::contexts::chain_driver::CosmosChainDriver;

pub struct ProvideCosmosBootstrapChainTypes;

#[cgp_provider(ChainTypeComponent)]
impl<Bootstrap> ProvideChainType<Bootstrap> for ProvideCosmosBootstrapChainTypes
where
    Bootstrap: Async,
{
    type Chain = CosmosChain;
}

#[cgp_provider(ChainDriverTypeComponent)]
impl<Bootstrap> ProvideChainDriverType<Bootstrap> for ProvideCosmosBootstrapChainTypes
where
    Bootstrap: HasChainType<Chain = CosmosChain>,
{
    type ChainDriver = CosmosChainDriver;
}
