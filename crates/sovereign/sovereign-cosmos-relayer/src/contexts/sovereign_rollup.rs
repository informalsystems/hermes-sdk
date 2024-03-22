use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent, HasComponents};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, ChainIdTypeComponent,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::encode::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
};
use hermes_relayer_components::error::impls::retry::ReturnRetryable;
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::traits::parse_events::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::poll_tx_response::{
    CanPollTxResponse, TxResponsePollerComponent,
};
use hermes_relayer_components::transaction::traits::query_tx_response::{
    CanQueryTxResponse, TxResponseQuerierComponent,
};
use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;
use hermes_relayer_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{ProvideRuntime, RuntimeTypeComponent};
use hermes_sovereign_client_components::sovereign::components::rollup::SovereignRollupClientComponents;
use hermes_sovereign_client_components::sovereign::traits::rollup::json_rpc_client::{
    JsonRpcClientGetter, JsonRpcClientTypeComponent,
};
use hermes_sovereign_client_components::sovereign::traits::rollup::publish_batch::{
    CanPublishTransactionBatch, TransactionBatchPublisherComponent,
};
use hermes_sovereign_client_components::sovereign::types::rollup_id::RollupId;
use hermes_sovereign_test_components::rollup::components::SovereignRollupTestComponents;
use hermes_test_components::chain::traits::assert::eventual_amount::{
    CanAssertEventualAmount, EventualAmountAsserterComponent,
};
use hermes_test_components::chain::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain::traits::queries::balance::{
    BalanceQuerierComponent, CanQueryBalance,
};
use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::wallet::WalletTypeComponent;
use jsonrpsee::http_client::HttpClient;

use crate::contexts::encoding::ProvideSovereignEncoding;
use crate::contexts::logger::ProvideSovereignLogger;

pub struct SovereignRollup {
    pub runtime: HermesRuntime,
    pub rpc_client: HttpClient,
}

pub struct SovereignRollupComponents;

impl HasComponents for SovereignRollup {
    type Components = SovereignRollupComponents;
}

delegate_components! {
    SovereignRollupComponents {
        ErrorTypeComponent:
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
        RetryableErrorComponent:
            ReturnRetryable<false>,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideSovereignLogger,
        [
            EncodingTypeComponent,
            EncodingGetterComponent,
            DefaultEncodingGetterComponent,
        ]:
            ProvideSovereignEncoding,
        [
            HeightTypeComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            TransactionTypeComponent,
            NonceTypeComponent,
            FeeTypeComponent,
            SignerTypeComponent,
            TransactionHashTypeComponent,
            TxResponseTypeComponent,
            JsonRpcClientTypeComponent,
            TransactionBatchPublisherComponent,
            TxResponseQuerierComponent,
            TxResponsePollerComponent,
            PollTimeoutGetterComponent,
            TxResponseAsEventsParserComponent,
        ]:
            SovereignRollupClientComponents,
        [
            AddressTypeComponent,
            DenomTypeComponent,
            AmountTypeComponent,
            WalletTypeComponent,
            BalanceQuerierComponent,
            EventualAmountAsserterComponent,
            PollAssertDurationGetterComponent,
        ]:
            SovereignRollupTestComponents,
    }
}

impl ProvideRuntime<SovereignRollup> for SovereignRollupComponents {
    fn runtime(rollup: &SovereignRollup) -> &HermesRuntime {
        &rollup.runtime
    }
}

impl JsonRpcClientGetter<SovereignRollup> for SovereignRollupComponents {
    fn json_rpc_client(rollup: &SovereignRollup) -> &HttpClient {
        &rollup.rpc_client
    }
}

impl ChainIdGetter<SovereignRollup> for SovereignRollupComponents {
    fn chain_id(_chain: &SovereignRollup) -> &RollupId {
        static DUMMY_ROLLUP_ID: RollupId = RollupId(0);

        &DUMMY_ROLLUP_ID
    }
}

pub trait CanUseSovereignRollup:
    CanQueryBalance
    + CanPublishTransactionBatch
    + CanQueryTxResponse
    + CanPollTxResponse
    + CanAssertEventualAmount
{
}

impl CanUseSovereignRollup for SovereignRollup {}
