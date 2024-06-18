use cgp_core::prelude::*;
use hermes_cosmos_test_components::chain::impls::transfer::timeout::IbcTransferTimeoutAfterSeconds;
use hermes_cosmos_test_components::chain::impls::types::address::ProvideStringAddress;
use hermes_test_components::chain::impls::assert::default_assert_duration::ProvideDefaultPollAssertDuration;
use hermes_test_components::chain::impls::assert::poll_assert_eventual_amount::PollAssertEventualAmount;
use hermes_test_components::chain::impls::default_memo::ProvideDefaultMemo;
use hermes_test_components::chain::impls::ibc_transfer::SendIbcTransferMessage;
use hermes_test_components::chain::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilderComponent;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain::traits::transfer::ibc_transfer::TokenIbcTransferrerComponent;
use hermes_test_components::chain::traits::transfer::string_memo::ProvideStringMemoType;
use hermes_test_components::chain::traits::transfer::timeout::IbcTransferTimeoutCalculatorComponent;
use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::memo::{
    DefaultMemoGetterComponent, MemoTypeComponent,
};
use hermes_test_components::chain::traits::types::wallet::WalletTypeComponent;

use crate::rollup::impls::ibc_transfer_message::BuildSovereignIbcTransferMessage;
use crate::rollup::impls::queries::balance::QuerySovereignBalance;
use crate::rollup::impls::types::amount::ProvideSovereignAmountType;
use crate::rollup::impls::types::denom::ProvideSovereignDenomType;
use crate::rollup::impls::types::wallet::ProvideSovereignWalletType;

pub struct SovereignRollupTestComponents;

delegate_components! {
    #[mark_component(IsSovereignRollupTestComponent)]
    SovereignRollupTestComponents {
        AddressTypeComponent: ProvideStringAddress,
        DenomTypeComponent: ProvideSovereignDenomType,
        AmountTypeComponent: ProvideSovereignAmountType,
        WalletTypeComponent: ProvideSovereignWalletType,
        BalanceQuerierComponent: QuerySovereignBalance,
        MemoTypeComponent:
            ProvideStringMemoType,
        DefaultMemoGetterComponent:
            ProvideDefaultMemo,
        TokenIbcTransferrerComponent:
            SendIbcTransferMessage,
        IbcTransferTimeoutCalculatorComponent:
            IbcTransferTimeoutAfterSeconds<90>,
        IbcTokenTransferMessageBuilderComponent:
            BuildSovereignIbcTransferMessage,
        EventualAmountAsserterComponent:
            PollAssertEventualAmount,
        PollAssertDurationGetterComponent:
            ProvideDefaultPollAssertDuration,
    }
}
