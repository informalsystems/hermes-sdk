use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_test_components::driver::traits::types::chain_driver_at::ChainDriverTypeProviderAtComponent;
use hermes_test_components::driver::traits::types::relay_driver_at::RelayDriverTypeProviderAtComponent;

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;

pub struct UseCosmosTestTypes;

delegate_components! {
    UseCosmosTestTypes {
        <I> ChainTypeProviderAtComponent<I>:
            UseType<CosmosChain>,
        <I> ChainDriverTypeProviderAtComponent<I>:
            UseType<CosmosChainDriver>,
        <I, J> RelayTypeProviderAtComponent<I, J>:
            UseType<CosmosRelay>,
        <I, J> BiRelayTypeProviderAtComponent<I, J>:
            UseType<CosmosBiRelay>,
        <I, J> RelayDriverTypeProviderAtComponent<I, J>:
            UseType<CosmosRelayDriver>,
    }
}
