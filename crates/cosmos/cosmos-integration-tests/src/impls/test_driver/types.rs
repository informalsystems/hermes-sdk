use cgp::core::field::Index;
use cgp::prelude::Async;
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

impl<Context, I: Async> ProvideChainTypeAt<Context, I> for ProvideCosmosTestTypes
where
    Context: Async,
{
    type Chain = CosmosChain;
}

impl<Context, I: Async> ProvideChainDriverTypeAt<Context, I> for ProvideCosmosTestTypes
where
    Context: HasChainTypeAt<I, Chain = CosmosChain>,
{
    type ChainDriver = CosmosChainDriver;
}

impl<Context, I: Async, J: Async> ProvideRelayTypeAt<Context, I, J> for ProvideCosmosTestTypes
where
    Context: HasChainTypeAt<I, Chain = CosmosChain> + HasChainTypeAt<J, Chain = CosmosChain>,
{
    type Relay = CosmosRelay;
}

impl<Context> ProvideBiRelayTypeAt<Context, Index<0>, Index<1>> for ProvideCosmosTestTypes
where
    Context: HasChainTypeAt<Index<0>, Chain = CosmosChain>
        + HasChainTypeAt<Index<1>, Chain = CosmosChain>
        + HasRelayTypeAt<Index<0>, Index<1>, Relay = CosmosRelay>
        + HasRelayTypeAt<Index<1>, Index<0>, Relay = CosmosRelay>,
{
    type BiRelay = CosmosBiRelay;
}

impl<Context, I: Async, J: Async> ProvideRelayDriverTypeAt<Context, I, J> for ProvideCosmosTestTypes
where
    Context: HasChainTypeAt<I, Chain = CosmosChain>
        + HasChainTypeAt<J, Chain = CosmosChain>
        + HasRelayTypeAt<I, J, Relay = CosmosRelay>
        + HasRelayTypeAt<J, I, Relay = CosmosRelay>
        + HasBiRelayTypeAt<I, J, BiRelay = CosmosBiRelay>,
{
    type RelayDriver = CosmosRelayDriver;
}
