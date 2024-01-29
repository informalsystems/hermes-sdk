use cgp_core::HasComponents;
use hermes_relayer_components::chain::traits::components::ack_packet_message_builder::{
    AckPacketMessageBuilder, CanBuildAckPacketMessage,
};
use hermes_relayer_components::chain::traits::components::ack_packet_payload_builder::{
    AckPacketPayloadBuilder, CanBuildAckPacketPayload,
};
use hermes_relayer_components::chain::traits::components::packet_fields_reader::{
    CanReadPacketFields, PacketFieldsReader,
};
use hermes_relayer_components::chain::traits::components::receive_packet_message_builder::{
    CanBuildReceivePacketMessage, ReceivePacketMessageBuilder,
};
use hermes_relayer_components::chain::traits::components::receive_packet_payload_builder::{
    CanBuildReceivePacketPayload, ReceivePacketPayloadBuilder,
};
use hermes_relayer_components::chain::traits::components::received_packet_querier::{
    CanQueryReceivedPacket, ReceivedPacketQuerier,
};
use hermes_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::{
    CanBuildTimeoutUnorderedPacketMessage, CanBuildTimeoutUnorderedPacketPayload,
    TimeoutUnorderedPacketMessageBuilder, TimeoutUnorderedPacketPayloadBuilder,
};
use hermes_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;

use crate::components::extra::chain::DelegatesToExtraChainComponents;
use crate::components::extra::closures::chain::message_sender::UseExtraChainComponentsForIbcMessageSender;

pub trait UseExtraChainComponentsForPacketRelayer<Counterparty>:
    CanLogChainPacket<Counterparty>
    + CanQueryReceivedPacket<Counterparty>
    + CanReadPacketFields<Counterparty>
    + CanBuildReceivePacketPayload<Counterparty>
    + CanBuildReceivePacketMessage<Counterparty>
    + CanBuildAckPacketPayload<Counterparty>
    + CanBuildAckPacketMessage<Counterparty>
    + CanBuildTimeoutUnorderedPacketPayload<Counterparty>
    + CanBuildTimeoutUnorderedPacketMessage<Counterparty>
    + UseExtraChainComponentsForIbcMessageSender<Counterparty>
where
    Counterparty: HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasIbcChainTypes<Self>
        + HasUpdateClientPayload<Self>
        + HasReceivePacketPayloadType<Self>
        + HasAckPacketPayloadType<Self>
        + HasTimeoutUnorderedPacketPayloadType<Self>,
{
}

impl<Chain, Counterparty, Components, BaseComponents>
    UseExtraChainComponentsForPacketRelayer<Counterparty> for Chain
where
    Chain: CanLogChainPacket<Counterparty>
        + HasIbcPacketTypes<Counterparty>
        + HasReceivePacketPayloadType<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasAckPacketPayloadType<Counterparty>
        + HasTimeoutUnorderedPacketPayloadType<Counterparty>
        + UseExtraChainComponentsForIbcMessageSender<Counterparty>
        + HasComponents<Components = Components>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasUpdateClientPayload<Chain>
        + HasAckPacketPayloadType<Chain>
        + HasTimeoutUnorderedPacketPayloadType<Chain>
        + HasReceivePacketPayloadType<Chain>,
    Components: HasComponents<Components = BaseComponents>
        + DelegatesToExtraChainComponents<BaseComponents>
        + PacketFieldsReader<Chain, Counterparty>
        + ReceivedPacketQuerier<Chain, Counterparty>
        + ReceivePacketPayloadBuilder<Chain, Counterparty>
        + ReceivePacketMessageBuilder<Chain, Counterparty>
        + AckPacketPayloadBuilder<Chain, Counterparty>
        + AckPacketMessageBuilder<Chain, Counterparty>
        + TimeoutUnorderedPacketPayloadBuilder<Chain, Counterparty>
        + TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty>,
{
}
