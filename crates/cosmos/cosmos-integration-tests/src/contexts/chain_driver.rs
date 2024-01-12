use alloc::sync::Arc;
use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_core::ProvideInner;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::chain::impls::address::ProvideStringAddress;
use hermes_cosmos_test_components::chain::impls::amount::ProvideU128AmountWithDenom;
use hermes_cosmos_test_components::chain::impls::chain_id::BuildCosmosChainIdFromString;
use hermes_cosmos_test_components::chain::impls::denom::ProvideIbcDenom;
use hermes_cosmos_test_components::chain::impls::wallet::ProvideCosmosTestWallet;
use hermes_relayer_components::chain::impls::forward::all::ForwardToInnerChain;
use hermes_relayer_components::chain::impls::forward::all::IsForwardToInnerChainComponent;
use hermes_test_components::chain::traits::build::ChainIdFromStringBuilderComponent;
use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::wallet::{
    WalletSignerComponent, WalletTypeComponent,
};
use hermes_test_components::driver::traits::types::chain::ChainGetter;
use hermes_test_components::driver::traits::types::chain::ProvideChainType;
use ibc_relayer::config::ChainConfig;
use tokio::process::Child;

#[derive(Clone)]
pub struct CosmosChainDriver {
    pub base_chain: CosmosChain,
    pub full_node_process: Arc<Child>,
    pub chain_config: ChainConfig,
}

pub struct CosmosChainDriverComponents;

impl HasComponents for CosmosChainDriver {
    type Components = CosmosChainDriverComponents;
}

delegate_all!(
    IsForwardToInnerChainComponent,
    ForwardToInnerChain,
    CosmosChainDriverComponents,
);

delegate_components! {
    CosmosChainDriverComponents {
        ErrorTypeComponent:
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
        [
            WalletTypeComponent,
            WalletSignerComponent,
        ]:
            ProvideCosmosTestWallet,
        ChainIdFromStringBuilderComponent:
            BuildCosmosChainIdFromString,
        AmountTypeComponent:
            ProvideU128AmountWithDenom,
        DenomTypeComponent:
            ProvideIbcDenom,
        AddressTypeComponent:
            ProvideStringAddress,
    }
}

impl<Driver> ProvideChainType<Driver> for CosmosChainDriverComponents
where
    Driver: Async,
{
    type Chain = CosmosChain;
}

impl ChainGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn chain(driver: &CosmosChainDriver) -> &CosmosChain {
        &driver.base_chain
    }
}

impl ProvideInner<CosmosChainDriver> for CosmosChainDriverComponents {
    type Inner = CosmosChain;

    fn inner(chain: &CosmosChainDriver) -> &Self::Inner {
        &chain.base_chain
    }
}
