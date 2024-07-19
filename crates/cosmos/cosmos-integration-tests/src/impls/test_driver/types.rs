use cgp_core::prelude::Async;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_relayer_components::multi::traits::birelay_at::{
    HasBiRelayTypeAt, ProvideBiRelayTypeAt,
};
use hermes_relayer_components::multi::traits::chain_at::{HasChainTypeAt, ProvideChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{HasRelayTypeAt, ProvideRelayTypeAt};
use hermes_test_components::driver::traits::types::chain_driver_at::ProvideChainDriverTypeAt;
use hermes_test_components::driver::traits::types::relay_driver_at::ProvideRelayDriverTypeAt;

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;

pub struct ProvideCosmosTestTypes;

impl<Context, const I: usize> ProvideChainTypeAt<Context, I> for ProvideCosmosTestTypes
where
    Context: Async,
{
    type Chain = CosmosChain;
}

impl<Context, const I: usize> ProvideChainDriverTypeAt<Context, I> for ProvideCosmosTestTypes
where
    Context: HasChainTypeAt<I, Chain = CosmosChain>,
{
    type ChainDriver = CosmosChainDriver;
}

impl<Context, const I: usize, const J: usize> ProvideRelayTypeAt<Context, I, J>
    for ProvideCosmosTestTypes
where
    Context: HasChainTypeAt<I, Chain = CosmosChain> + HasChainTypeAt<J, Chain = CosmosChain>,
{
    type Relay = CosmosRelay;
}

impl<Context> ProvideBiRelayTypeAt<Context, 0, 1> for ProvideCosmosTestTypes
where
    Context: HasChainTypeAt<0, Chain = CosmosChain>
        + HasChainTypeAt<1, Chain = CosmosChain>
        + HasRelayTypeAt<0, 1, Relay = CosmosRelay>
        + HasRelayTypeAt<1, 0, Relay = CosmosRelay>,
{
    type BiRelay = CosmosBiRelay;
}

impl<Context, const I: usize, const J: usize> ProvideRelayDriverTypeAt<Context, I, J>
    for ProvideCosmosTestTypes
where
    Context: HasChainTypeAt<I, Chain = CosmosChain>
        + HasChainTypeAt<J, Chain = CosmosChain>
        + HasRelayTypeAt<I, J, Relay = CosmosRelay>
        + HasRelayTypeAt<J, I, Relay = CosmosRelay>
        + HasBiRelayTypeAt<I, J, BiRelay = CosmosBiRelay>,
{
    type RelayDriver = CosmosRelayDriver;
}
