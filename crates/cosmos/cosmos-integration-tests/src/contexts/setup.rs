use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_test_components::driver::traits::types::chain_at::ProvideChainTypeAt;
use hermes_test_components::setup::components::binary_channel::BinaryChannelTestComponents;
use hermes_test_components::setup::components::binary_channel::IsBinaryChannelTestComponent;

pub struct CosmosSetup;

pub struct CosmosSetupComponents;

delegate_all!(
    IsBinaryChannelTestComponent,
    BinaryChannelTestComponents,
    CosmosSetupComponents,
);

delegate_components! {
    CosmosSetupComponents {
        ErrorTypeComponent: ProvideEyreError,
    }
}

impl<Setup, const I: usize> ProvideChainTypeAt<Setup, I> for CosmosSetupComponents
where
    Setup: Async,
{
    type Chain = CosmosChain;
}
