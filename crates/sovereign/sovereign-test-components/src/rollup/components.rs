use cgp_core::prelude::*;
use hermes_cosmos_test_components::chain::impls::types::address::ProvideStringAddress;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::wallet::WalletTypeComponent;

use crate::rollup::impls::queries::balance::QuerySovereignBalance;
use crate::rollup::impls::types::amount::ProvideSovereignAmountType;
use crate::rollup::impls::types::denom::ProvideSovereignDenomType;
use crate::rollup::impls::types::wallet::ProvideSovereignWalletType;

pub struct SovereignRollupTestComponents;

delegate_components! {
    #[mark_component(IsSovereignTestComponent)]
    SovereignRollupTestComponents {
        AddressTypeComponent: ProvideStringAddress,
        DenomTypeComponent: ProvideSovereignDenomType,
        AmountTypeComponent: ProvideSovereignAmountType,
        WalletTypeComponent: ProvideSovereignWalletType,
        BalanceQuerierComponent: QuerySovereignBalance,
    }
}
