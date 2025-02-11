use cgp::prelude::*;
use hermes_logging_components::impls::delegate::DelegateLogger;
use hermes_logging_components::impls::global::GetGlobalLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetter, GlobalLoggerGetterComponent, HasLoggerType, LoggerGetterComponent,
    LoggerTypeComponent, ProvideLoggerType,
};
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_logging_components::types::level::{
    LevelDebug, LevelError, LevelInfo, LevelTrace, LevelWarn,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::relay::impls::packet_clearers::receive_packet::LogClearPacketError;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::LogRelayPacketAction;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::LogRelayPacketStatus;
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
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_components_extra::batch::worker::LogBatchWorker;
use hermes_tracing_logging_components::contexts::logger::TracingLogger;

pub struct HermesLogger;

pub struct HermesLoggerComponents;

pub struct HermesLogHandlers;

impl HasComponents for HermesLogger {
    type Components = HermesLoggerComponents;
}

delegate_components! {
    HermesLoggerComponents {
        LoggerComponent: DelegateLogger<HermesLogHandlers>,
    }
}

delegate_components! {
    HermesLogHandlers {
        [
            (),
            LevelTrace,
            LevelDebug,
            LevelInfo,
            LevelWarn,
            LevelError,
            <'a, Chain: HasSignerType + HasNonceType + HasMessageType,>
                LogSendMessagesWithSignerAndNonce<'a, Chain>,
            <'a, Chain: HasTransactionHashType>
                TxNoResponseError<'a, Chain>,
            <'a, Chain: HasTransactionHashType + HasAsyncErrorType>
                LogRetryQueryTxResponse<'a, Chain>,
            <'a, Relay: HasRelayChains>
                LogSkipRelayLockedPacket<'a, Relay>,
            <'a, Relay: HasRelayChains>
                LogRelayPacketAction<'a, Relay>,
            <'a, Relay: HasRelayChains>
                LogClearPacketError<'a, Relay>,
            <'a, Relay: HasRelayChains>
                LogRelayPacketStatus<'a, Relay>,
            <'a, Relay: HasTargetChainTypes<Target, CounterpartyChain: HasHeightType>, Target: RelayTarget>
                LogSkipBuildUpdateClientMessage<'a, Relay, Target>,
            <'a, Relay: HasTargetChainTypes<Target, CounterpartyChain: HasHeightType>, Target: RelayTarget>
                LogWaitUpdateClientHeightStatus<'a, Relay, Target>,
            <'a, Relay: HasRelayChains, Target: RelayTarget>
                LogBatchWorker<'a, Relay, Target>,
        ]: TracingLogger,
    }
}

pub struct ProvideHermesLogger;

delegate_components! {
    ProvideHermesLogger {
        LoggerGetterComponent: GetGlobalLogger,
    }
}

#[cgp_provider(LoggerTypeComponent)]
impl<Context> ProvideLoggerType<Context> for ProvideHermesLogger
where
    Context: Async,
{
    type Logger = HermesLogger;
}

#[cgp_provider(GlobalLoggerGetterComponent)]
impl<Context> GlobalLoggerGetter<Context> for ProvideHermesLogger
where
    Context: HasLoggerType<Logger = HermesLogger>,
{
    fn global_logger() -> &'static HermesLogger {
        &HermesLogger
    }
}
