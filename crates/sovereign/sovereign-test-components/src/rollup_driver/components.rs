use cgp_core::prelude::*;
use hermes_cosmos_test_components::chain_driver::impls::address::ProvideStringAddress;
use hermes_test_components::chain_driver::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain_driver::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain_driver::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain_driver::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain_driver::traits::types::wallet::WalletTypeComponent;

use crate::rollup_driver::impls::queries::balance::QuerySovereignBalance;
use crate::rollup_driver::impls::types::amount::ProvideSovereignAmountType;
use crate::rollup_driver::impls::types::denom::ProvideSovereignDenomType;
use crate::rollup_driver::impls::types::wallet::ProvideSovereignWalletType;

pub struct SovereignTestComponents;

delegate_components! {
    #[mark_component(IsSovereignTestComponent)]
    SovereignTestComponents {
        AddressTypeComponent: ProvideStringAddress,
        DenomTypeComponent: ProvideSovereignDenomType,
        AmountTypeComponent: ProvideSovereignAmountType,
        WalletTypeComponent: ProvideSovereignWalletType,
        BalanceQuerierComponent: QuerySovereignBalance,
    }
}
