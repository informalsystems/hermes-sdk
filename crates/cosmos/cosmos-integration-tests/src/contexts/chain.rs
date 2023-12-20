use cgp_core::prelude::*;
use cosmos_test_components::chain::impls::address::ProvideStringAddress;
use cosmos_test_components::chain::impls::amount::ProvideU128AmountWithDenom;
use cosmos_test_components::chain::impls::chain_id::BuildCosmosChainIdFromString;
use cosmos_test_components::chain::impls::denom::ProvideIbcDenom;
use cosmos_test_components::chain::impls::wallet::ProvideCosmosTestWallet;
use hermes_cosmos_client_components::components::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use ibc_test_components::chain::traits::build::ChainIdFromStringBuilderComponent;
use ibc_test_components::chain::traits::types::address::AddressTypeComponent;
use ibc_test_components::chain::traits::types::amount::AmountTypeComponent;
use ibc_test_components::chain::traits::types::denom::DenomTypeComponent;
use ibc_test_components::chain::traits::types::wallet::{
    WalletSignerComponent, WalletTypeComponent,
};

pub struct CosmosTestChain;

pub struct CosmosTestChainComponents;

impl HasComponents for CosmosTestChain {
    type Components = CosmosTestChainComponents;
}

delegate_components! {
    CosmosTestChainComponents {
        [
            ChainIdTypeProviderComponent,
        ]:
            ProvideCosmosChainTypes,
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
