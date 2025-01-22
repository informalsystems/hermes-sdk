use cgp::prelude::HasComponents;
use hermes_relayer_components::chain::traits::extract_data::CanExtractFromEvent;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::{
    AckPacketMessageBuilder, CanBuildAckPacketMessage,
};
use hermes_relayer_components::chain::traits::message_builders::receive_packet::{
    CanBuildReceivePacketMessage, ReceivePacketMessageBuilder,
};
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::{
    CanBuildTimeoutUnorderedPacketMessage, TimeoutUnorderedPacketMessageBuilder,
};
use hermes_relayer_components::chain::traits::packet::fields::CanReadPacketFields;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::{
    AckPacketPayloadBuilder, CanBuildAckPacketPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::{
    CanBuildReceivePacketPayload, ReceivePacketPayloadBuilder,
};
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::{
    CanBuildTimeoutUnorderedPacketPayload, TimeoutUnorderedPacketPayloadBuilder,
};
use hermes_relayer_components::chain::traits::queries::packet_is_received::{
    CanQueryPacketIsReceived, ReceivedPacketQuerier,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;

use crate::components::extra::closures::chain::message_sender::UseExtraChainComponentsForIbcMessageSender;

pub trait UseExtraChainComponentsForPacketRelayer<Counterparty>:
    CanQueryPacketIsReceived<Counterparty>
    + CanReadPacketFields<Counterparty>
    + CanBuildReceivePacketPayload<Counterparty>
    + CanBuildReceivePacketMessage<Counterparty>
    + CanBuildAckPacketPayload<Counterparty>
    + CanBuildAckPacketMessage<Counterparty>
    + CanBuildTimeoutUnorderedPacketPayload<Counterparty>
    + CanBuildTimeoutUnorderedPacketMessage<Counterparty>
    + HasWriteAckEvent<Counterparty>
    + CanExtractFromEvent<Self::WriteAckEvent>
    + UseExtraChainComponentsForIbcMessageSender<Counterparty>
where
    Counterparty: HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasIbcChainTypes<Self>
        + HasOutgoingPacketType<Self>
        + HasUpdateClientPayloadType<Self>
        + HasReceivePacketPayloadType<Self>
        + HasAckPacketPayloadType<Self>
        + HasTimeoutUnorderedPacketPayloadType<Self>,
{
}

impl<Chain, Counterparty, Components> UseExtraChainComponentsForPacketRelayer<Counterparty>
    for Chain
where
    Chain: HasIbcChainTypes<Counterparty>
        + HasOutgoingPacketType<Counterparty>
        + HasReceivePacketPayloadType<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasAckPacketPayloadType<Counterparty>
        + CanReadPacketFields<Counterparty>
        + HasTimeoutUnorderedPacketPayloadType<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + CanExtractFromEvent<Chain::WriteAckEvent>
        + UseExtraChainComponentsForIbcMessageSender<Counterparty>
        + HasComponents<Components = Components>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasOutgoingPacketType<Chain>
        + HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasUpdateClientPayloadType<Chain>
        + HasAckPacketPayloadType<Chain>
        + HasTimeoutUnorderedPacketPayloadType<Chain>
        + HasReceivePacketPayloadType<Chain>,
    Components: ReceivedPacketQuerier<Chain, Counterparty>
        + ReceivePacketPayloadBuilder<Chain, Counterparty>
        + ReceivePacketMessageBuilder<Chain, Counterparty>
        + AckPacketPayloadBuilder<Chain, Counterparty>
        + AckPacketMessageBuilder<Chain, Counterparty>
        + TimeoutUnorderedPacketPayloadBuilder<Chain, Counterparty>
        + TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty>,
{
}
