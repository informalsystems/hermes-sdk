use cgp::prelude::*;
use hermes_core::relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_core::test_components::driver::traits::{
    ChainDriverTypeProviderAtComponent, RelayDriverTypeProviderAtComponent,
};
use hermes_cosmos_relayer::contexts::{CosmosBiRelay, CosmosChain, CosmosRelay};

use crate::contexts::{CosmosChainDriver, CosmosRelayDriver};

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
