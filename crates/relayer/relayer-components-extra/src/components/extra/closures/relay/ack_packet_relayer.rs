use cgp::core::component::HasComponents;
use cgp::core::error::{CanRaiseAsyncError, ErrorOf, ErrorRaiser, HasAsyncErrorType};
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::packet::fields::CanReadPacketFields;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::CanBuildAckPacketPayload;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::queries::consensus_state::CanQueryConsensusState;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use hermes_relayer_components::chain::traits::send_message::{
    CanSendMessages, EmptyMessageResponse,
};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::ibc::{
    HasCounterpartyMessageHeight, HasIbcChainTypes,
};
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::components::default::relay::MainSink;
use hermes_relayer_components::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use hermes_relayer_components::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use hermes_relayer_components::relay::traits::chains::{HasRelayChains, HasRelayClientIds};
use hermes_relayer_components::relay::traits::ibc_message_sender::CanSendSingleIbcMessage;
use hermes_relayer_components::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use hermes_relayer_components::relay::traits::target::{
    HasDestinationTargetChainTypes, HasSourceTargetChainTypes, SourceTarget,
};
use hermes_runtime_components::traits::channel::CanUseChannels;
use hermes_runtime_components::traits::channel_once::{CanCreateChannelsOnce, CanUseChannelsOnce};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::components::extra::relay::DelegatesToExtraRelayPreset;

pub trait CanUseExtraAckPacketRelayer: UseExtraAckPacketRelayer
where
    Self::DstChain: HasWriteAckEvent<Self::SrcChain>,
{
}

pub trait UseExtraAckPacketRelayer: CanRelayAckPacket
where
    Self::DstChain: HasWriteAckEvent<Self::SrcChain>,
{
}

impl<Relay, SrcChain, DstChain, Components, Logger> UseExtraAckPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        + CanSendSingleIbcMessage<MainSink, SourceTarget>
        + HasLogger<Logger = Logger>
        + HasComponents<Components = Components>,
    SrcChain: HasAsyncErrorType
        + HasRuntime
        + HasChainId
        + CanSendMessages
        + CanQueryChainStatus
        + HasIbcChainTypes<DstChain>
        + HasConsensusStateType<DstChain>
        + HasCounterpartyMessageHeight<DstChain>
        + CanReadPacketFields<DstChain>
        + CanQueryClientState<DstChain>
        + CanQueryConsensusState<DstChain>
        + CanQueryConsensusStateHeight<DstChain>
        + CanBuildAckPacketMessage<DstChain>
        + CanBuildUpdateClientMessage<DstChain>
        + CanRaiseAsyncError<ErrorOf<SrcChain::Runtime>>,
    DstChain: HasAsyncErrorType
        + HasRuntime
        + HasChainId
        + CanQueryChainStatus
        + HasIbcChainTypes<SrcChain>
        + HasClientStateFields<SrcChain>
        + HasConsensusStateType<SrcChain>
        + HasWriteAckEvent<SrcChain>
        + CanBuildAckPacketPayload<SrcChain>
        + CanBuildUpdateClientPayload<SrcChain>
        + CanRaiseAsyncError<ErrorOf<DstChain>>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    SrcChain::Runtime: CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    DstChain::Runtime: CanSleep,
    Logger: for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, SourceTarget>>
        + for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, SourceTarget>>,
    Components: DelegatesToExtraRelayPreset
        + ErrorRaiser<Relay, EmptyMessageResponse>
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
