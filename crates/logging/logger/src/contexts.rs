use cgp::prelude::*;
use hermes_logging_components::impls::delegate::DelegateLogger;
use hermes_logging_components::impls::global::GetGlobalLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetter, GlobalLoggerGetterComponent, HasLoggerType, LoggerGetterComponent,
    LoggerTypeProviderComponent,
};
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_logging_components::types::level::{
    LevelDebug, LevelError, LevelInfo, LevelTrace, LevelWarn,
};
use hermes_relayer_components::birelay::impls::auto_birelay::LogAutoBiRelay;
use hermes_relayer_components::birelay::traits::HasBiRelayTypes;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::error::impls::retry::LogPerformRetry;
use hermes_relayer_components::relay::impls::auto_relayers::poll_event::LogAutoRelayWithHeights;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::LogRelayPacketAction;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::LogRelayPacketStatus;
use hermes_relayer_components::relay::impls::update_client::build::LogClientUpdateMessage;
use hermes_relayer_components::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use hermes_relayer_components::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::target::{HasTargetChainTypes, RelayTarget};
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    LogRetryQueryTxResponse, TxNoResponseError,
};
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTxHashType;
use hermes_relayer_components_extra::batch::worker::LogBatchWorker;
use hermes_tracing_logging_components::contexts::logger::TracingLogger;

#[cgp_context(HermesLoggerComponents)]
pub struct HermesLogger;

pub struct HandleHermesLogs;

delegate_components! {
    HermesLoggerComponents {
        LoggerComponent: DelegateLogger<HandleHermesLogs>,
    }
}

delegate_components! {
    HandleHermesLogs {
        [
            (),
            LevelTrace,
            LevelDebug,
            LevelInfo,
            LevelWarn,
            LevelError,
            <'a, Context: HasErrorType>
                LogPerformRetry<'a, Context>,
            <'a, Chain: HasSignerType + HasNonceType + HasMessageType,>
                LogSendMessagesWithSignerAndNonce<'a, Chain>,
            <'a, Chain: HasTxHashType>
                TxNoResponseError<'a, Chain>,
            <'a, Chain: HasTxHashType + HasAsyncErrorType>
                LogRetryQueryTxResponse<'a, Chain>,
            <'a, Relay: HasRelayChains>
                LogSkipRelayLockedPacket<'a, Relay>,
            <'a, Relay: HasRelayChains>
                LogRelayPacketAction<'a, Relay>,
            <'a, Relay: HasRelayChains>
                LogRelayPacketStatus<'a, Relay>,
            <'a, Relay: HasTargetChainTypes<Target, CounterpartyChain: HasHeightType>, Target: RelayTarget>
                LogSkipBuildUpdateClientMessage<'a, Relay, Target>,
            <'a, Relay: HasTargetChainTypes<Target, TargetChain: HasClientIdType<Relay::CounterpartyChain>, CounterpartyChain: HasHeightType>, Target: RelayTarget>
                LogClientUpdateMessage<'a, Relay, Target>,
            <'a, Relay: HasTargetChainTypes<Target, CounterpartyChain: HasHeightType>, Target: RelayTarget>
                LogWaitUpdateClientHeightStatus<'a, Relay, Target>,
            <'a, Relay: HasRelayChains, Target: RelayTarget>
                LogBatchWorker<'a, Relay, Target>,
            <'a, BiRelay: HasBiRelayTypes<ChainA: HasHeightType, ChainB: HasHeightType>>
                LogAutoBiRelay<'a, BiRelay>,
            <'a, Relay: HasTargetChainTypes<Target, TargetChain: HasHeightType>, Target: RelayTarget>
                LogAutoRelayWithHeights<'a, Relay, Target>,
        ]: TracingLogger,
    }
}

pub struct UseHermesLogger;

delegate_components! {
    UseHermesLogger {
        LoggerTypeProviderComponent: UseType<HermesLogger>,
        LoggerGetterComponent: GetGlobalLogger,
    }
}

#[cgp_provider(GlobalLoggerGetterComponent)]
impl<Context> GlobalLoggerGetter<Context> for UseHermesLogger
where
    Context: HasLoggerType<Logger = HermesLogger>,
{
    fn global_logger() -> &'static HermesLogger {
        &HermesLogger
    }
}
