use cgp_core::prelude::*;
use hermes_test_components::chain::impls::default_memo::ProvideDefaultMemo;
use hermes_test_components::chain::impls::ibc_transfer::SendIbcTransferMessage;
use hermes_test_components::chain::traits::chain_id::ChainIdFromStringBuilderComponent;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilderComponent;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain::traits::transfer::amount::IbcTransferredAmountConverterComponent;
use hermes_test_components::chain::traits::transfer::ibc_transfer::TokenIbcTransferrerComponent;
use hermes_test_components::chain::traits::transfer::string_memo::ProvideStringMemoType;
use hermes_test_components::chain::traits::transfer::timeout::IbcTransferTimeoutCalculatorComponent;
use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain::traits::types::amount::{
    AmountMethodsComponent, AmountTypeComponent,
};
use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::memo::{
    DefaultMemoGetterComponent, MemoTypeComponent,
};
use hermes_test_components::chain::traits::types::wallet::{
    WalletSignerComponent, WalletTypeComponent,
};

use crate::chain::impls::chain_id::BuildCosmosChainIdFromString;
use crate::chain::impls::messages::ibc_transfer::BuildCosmosIbcTransferMessage;
use crate::chain::impls::queries::balance::QueryCosmosBalance;
use crate::chain::impls::transfer::amount::ConvertCosmosIbcAmount;
use crate::chain::impls::transfer::timeout::IbcTransferTimeoutAfterSeconds;
use crate::chain::impls::types::address::ProvideStringAddress;
use crate::chain::impls::types::amount::ProvideU128AmountWithDenom;
use crate::chain::impls::types::denom::ProvideIbcDenom;
use crate::chain::impls::types::wallet::ProvideCosmosTestWallet;

pub struct CosmmosChainTestComponents;

delegate_components! {
    CosmmosChainTestComponents {
        [
            WalletTypeComponent,
            WalletSignerComponent,
        ]:
            ProvideCosmosTestWallet,
        ChainIdFromStringBuilderComponent:
            BuildCosmosChainIdFromString,
        [
            AmountTypeComponent,
            AmountMethodsComponent,
        ]:
            ProvideU128AmountWithDenom,
        DenomTypeComponent:
            ProvideIbcDenom,
        AddressTypeComponent:
            ProvideStringAddress,
        MemoTypeComponent:
            ProvideStringMemoType,
        DefaultMemoGetterComponent:
            ProvideDefaultMemo,
        TokenIbcTransferrerComponent:
            SendIbcTransferMessage,
        IbcTransferTimeoutCalculatorComponent:
            IbcTransferTimeoutAfterSeconds<90>,
        IbcTokenTransferMessageBuilderComponent:
            BuildCosmosIbcTransferMessage,
        IbcTransferredAmountConverterComponent:
            ConvertCosmosIbcAmount,
        BalanceQuerierComponent:
            QueryCosmosBalance,
    }
}
