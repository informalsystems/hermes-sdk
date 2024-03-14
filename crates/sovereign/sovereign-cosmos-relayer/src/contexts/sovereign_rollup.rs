use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_core::HasComponents;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::encode::traits::has_encoding::DefaultEncodingGetterComponent;
use hermes_relayer_components::encode::traits::has_encoding::EncodingGetterComponent;
use hermes_relayer_components::encode::traits::has_encoding::EncodingTypeComponent;
use hermes_relayer_components::logger::traits::has_logger::LoggerFieldComponent;
use hermes_relayer_components::logger::traits::has_logger::LoggerTypeComponent;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_components::transaction::components::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::traits::components::tx_response_poller::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::components::tx_response_poller::TxResponsePollerComponent;
use hermes_relayer_components::transaction::traits::components::tx_response_querier::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::components::tx_response_querier::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::event::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::types::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::TxResponseTypeComponent;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_client_components::sovereign::components::rollup::SovereignRollupClientComponents;
use hermes_sovereign_client_components::sovereign::traits::rollup::json_rpc_client::JsonRpcClientGetter;
use hermes_sovereign_client_components::sovereign::traits::rollup::json_rpc_client::JsonRpcClientTypeComponent;
use hermes_sovereign_client_components::sovereign::traits::rollup::publish_batch::CanPublishTransactionBatch;
use hermes_sovereign_client_components::sovereign::traits::rollup::publish_batch::TransactionBatchPublisherComponent;
use hermes_sovereign_client_components::sovereign::types::rollup_id::RollupId;
use hermes_sovereign_test_components::rollup::components::SovereignRollupTestComponents;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::wallet::WalletTypeComponent;
use jsonrpsee::http_client::HttpClient;

use crate::contexts::encoding::ProvideSovereignEncoding;

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
        RuntimeTypeComponent:
            ProvideTokioRuntimeType,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
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
