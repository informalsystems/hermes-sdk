use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent, HasComponents};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use ed25519_dalek::SigningKey;
use futures::lock::Mutex;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
};
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
use hermes_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, ChainIdTypeComponent, HasChainId,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::error::impls::retry::ReturnRetryable;
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::traits::encode_tx::{CanEncodeTx, TxEncoderComponent};
use hermes_relayer_components::transaction::traits::estimate_tx_fee::{
    CanEstimateTxFee, TxFeeEstimatorComponent,
};
use hermes_relayer_components::transaction::traits::nonce::allocate_nonce::{
    CanAllocateNonce, NonceAllocatorComponent,
};
use hermes_relayer_components::transaction::traits::nonce::nonce_guard::{
    HasNonceGuard, NonceGuardComponent,
};
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::{
    HasMutexForNonceAllocation, ProvideMutexForNonceAllocation,
};
use hermes_relayer_components::transaction::traits::nonce::query_nonce::{
    CanQueryNonce, NonceQuerierComponent,
};
use hermes_relayer_components::transaction::traits::parse_events::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::poll_tx_response::{
    CanPollTxResponse, TxResponsePollerComponent,
};
use hermes_relayer_components::transaction::traits::query_tx_response::{
    CanQueryTxResponse, TxResponseQuerierComponent,
};
use hermes_relayer_components::transaction::traits::send_messages_with_signer::{
    CanSendMessagesWithSigner, MessagesWithSignerSenderComponent,
};
use hermes_relayer_components::transaction::traits::send_messages_with_signer_and_nonce::{
    CanSendMessagesWithSignerAndNonce, MessagesWithSignerAndNonceSenderComponent,
};
use hermes_relayer_components::transaction::traits::simulation_fee::{
    FeeForSimulationGetterComponent, HasFeeForSimulation,
};
use hermes_relayer_components::transaction::traits::submit_tx::{
    CanSubmitTx, TxSubmitterComponent,
};
use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::mutex::{HasMutex, MutexGuardOf};
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use hermes_sovereign_rollup_components::components::rollup::SovereignRollupClientComponents;
use hermes_sovereign_rollup_components::traits::json_rpc_client::{
    JsonRpcClientGetter, JsonRpcClientTypeComponent,
};
use hermes_sovereign_rollup_components::traits::publish_batch::{
    CanPublishTransactionBatch, TransactionBatchPublisherComponent,
};
use hermes_sovereign_rollup_components::types::rollup_id::RollupId;
use hermes_sovereign_rollup_components::types::tx::nonce_guard::SovereignNonceGuard;
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
    pub nonce_mutex: Mutex<()>,
}

impl SovereignRollup {
    pub fn new(runtime: HermesRuntime, rpc_client: HttpClient) -> Self {
        Self {
            runtime,
            rpc_client,
            nonce_mutex: Mutex::new(()),
        }
    }
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
            NonceGuardComponent,
            FeeTypeComponent,
            SignerTypeComponent,
            TransactionHashTypeComponent,
            TxResponseTypeComponent,

            NonceAllocatorComponent,
            MessageSenderComponent,
            MessagesWithSignerSenderComponent,
            MessagesWithSignerAndNonceSenderComponent,
            TxResponsePollerComponent,

            JsonRpcClientTypeComponent,
            TxEncoderComponent,
            TxFeeEstimatorComponent,
            FeeForSimulationGetterComponent,
            TxSubmitterComponent,
            NonceQuerierComponent,
            TransactionBatchPublisherComponent,
            TxResponseQuerierComponent,
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

impl RuntimeGetter<SovereignRollup> for SovereignRollupComponents {
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

impl ProvideMutexForNonceAllocation<SovereignRollup> for SovereignRollupComponents {
    fn mutex_for_nonce_allocation<'a>(
        rollup: &'a SovereignRollup,
        _signer: &SigningKey,
    ) -> &'a Mutex<()> {
        &rollup.nonce_mutex
    }

    fn mutex_to_nonce_guard<'a>(
        mutex_guard: MutexGuardOf<'a, HermesRuntime, ()>,
        nonce: u64,
    ) -> SovereignNonceGuard<'a> {
        SovereignNonceGuard { mutex_guard, nonce }
    }
}

pub trait CanUseSovereignRollup:
    CanQueryBalance
    + HasChainId
    + CanEncodeTx
    + CanEstimateTxFee
    + HasFeeForSimulation
    + CanSubmitTx
    + HasNonceGuard
    + HasMutexForNonceAllocation
    + CanQueryNonce
    + CanAllocateNonce
    + CanSendMessagesWithSigner
    + CanSendMessagesWithSignerAndNonce
    + CanPublishTransactionBatch
    + CanQueryTxResponse
    + CanPollTxResponse
    + CanAssertEventualAmount
    + HasLogger
where
    Self::Runtime: HasMutex,
{
}

impl CanUseSovereignRollup for SovereignRollup {}
