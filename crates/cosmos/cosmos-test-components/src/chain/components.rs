use cgp::prelude::*;
use hermes_test_components::chain::impls::assert::default_assert_duration::ProvideDefaultPollAssertDuration;
use hermes_test_components::chain::impls::assert::poll_assert_eventual_amount::PollAssertEventualAmount;
use hermes_test_components::chain::impls::default_memo::ProvideDefaultMemo;
use hermes_test_components::chain::impls::ibc_transfer::SendIbcTransferMessage;
pub use hermes_test_components::chain::traits::assert::eventual_amount::EventualAmountAsserterComponent;
pub use hermes_test_components::chain::traits::assert::poll_assert::PollAssertDurationGetterComponent;
pub use hermes_test_components::chain::traits::chain_id::ChainIdFromStringBuilderComponent;
pub use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilderComponent;
pub use hermes_test_components::chain::traits::proposal::messages::deposit::DepositProposalMessageBuilderComponent;
pub use hermes_test_components::chain::traits::proposal::messages::vote::VoteProposalMessageBuilderComponent;
pub use hermes_test_components::chain::traits::proposal::poll_status::ProposalStatusPollerComponent;
pub use hermes_test_components::chain::traits::proposal::query_status::ProposalStatusQuerierComponent;
pub use hermes_test_components::chain::traits::proposal::types::proposal_id::ProposalIdTypeComponent;
pub use hermes_test_components::chain::traits::proposal::types::proposal_status::ProposalStatusTypeComponent;
pub use hermes_test_components::chain::traits::proposal::types::vote::ProposalVoteTypeComponent;
pub use hermes_test_components::chain::traits::queries::balance::BalanceQuerierComponent;
pub use hermes_test_components::chain::traits::transfer::amount::IbcTransferredAmountConverterComponent;
pub use hermes_test_components::chain::traits::transfer::ibc_transfer::TokenIbcTransferrerComponent;
pub use hermes_test_components::chain::traits::transfer::string_memo::ProvideStringMemoType;
pub use hermes_test_components::chain::traits::transfer::timeout::IbcTransferTimeoutCalculatorComponent;
pub use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
pub use hermes_test_components::chain::traits::types::amount::{
    AmountMethodsComponent, AmountTypeComponent,
};
pub use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
pub use hermes_test_components::chain::traits::types::memo::{
    DefaultMemoGetterComponent, MemoTypeComponent,
};
pub use hermes_test_components::chain::traits::types::wallet::{
    WalletSignerComponent, WalletTypeComponent,
};

use crate::chain::impls::chain_id::BuildCosmosChainIdFromString;
use crate::chain::impls::messages::ibc_transfer::BuildCosmosIbcTransferMessage;
use crate::chain::impls::proposal::messages::deposit::BuildDepositProposalMessage;
use crate::chain::impls::proposal::messages::vote::BuildVoteProposalMessage;
use crate::chain::impls::proposal::poll_status::PollProposalStatus;
use crate::chain::impls::proposal::query_status::QueryProposalStatusWithGrpc;
use crate::chain::impls::queries::balance::QueryCosmosBalance;
use crate::chain::impls::transfer::amount::ConvertCosmosIbcAmount;
use crate::chain::impls::transfer::timeout::IbcTransferTimeoutAfterSeconds;
use crate::chain::impls::types::address::ProvideStringAddress;
use crate::chain::impls::types::amount::ProvideU128AmountWithDenom;
use crate::chain::impls::types::denom::ProvideIbcDenom;
use crate::chain::impls::types::proposal::ProvideCosmosProposalTypes;
use crate::chain::impls::types::wallet::ProvideCosmosTestWallet;

cgp_preset! {
    CosmosChainTestPreset {
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
        [
            ProposalIdTypeComponent,
            ProposalStatusTypeComponent,
            ProposalVoteTypeComponent,
        ]:
            ProvideCosmosProposalTypes,
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
        EventualAmountAsserterComponent:
            PollAssertEventualAmount,
        PollAssertDurationGetterComponent:
            ProvideDefaultPollAssertDuration,
        ProposalStatusQuerierComponent:
            QueryProposalStatusWithGrpc,
        ProposalStatusPollerComponent:
            PollProposalStatus,
        DepositProposalMessageBuilderComponent:
            BuildDepositProposalMessage,
        VoteProposalMessageBuilderComponent:
            BuildVoteProposalMessage,
    }
}
