use cgp_core::HasComponents;
use ibc_relayer_components::chain::traits::components::ack_packet_message_builder::{
    AckPacketMessageBuilder, CanBuildAckPacketMessage,
};
use ibc_relayer_components::chain::traits::components::ack_packet_payload_builder::{
    AckPacketPayloadBuilder, CanBuildAckPacketPayload,
};
use ibc_relayer_components::chain::traits::components::packet_fields_reader::{
    CanReadPacketFields, PacketFieldsReader,
};
use ibc_relayer_components::chain::traits::components::receive_packet_message_builder::{
    CanBuildReceivePacketMessage, ReceivePacketMessageBuilder,
};
use ibc_relayer_components::chain::traits::components::receive_packet_payload_builder::{
    CanBuildReceivePacketPayload, ReceivePacketPayloadBuilder,
};
use ibc_relayer_components::chain::traits::components::received_packet_querier::{
    CanQueryReceivedPacket, ReceivedPacketQuerier,
};
use ibc_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::{
    CanBuildTimeoutUnorderedPacketMessage, CanBuildTimeoutUnorderedPacketPayload,
    TimeoutUnorderedPacketMessageBuilder, TimeoutUnorderedPacketPayloadBuilder,
};
use ibc_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use ibc_relayer_components::chain::traits::types::client_state::HasClientStateType;
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;
use ibc_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayload;
use ibc_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use ibc_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;

use crate::components::extra::chain::ExtraChainComponents;
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
        + HasReceivePacketPayload<Self>
        + HasAckPacketPayload<Self>
        + HasTimeoutUnorderedPacketPayload<Self>,
{
}

impl<Chain, Counterparty, ChainComponents> UseExtraChainComponentsForPacketRelayer<Counterparty>
    for Chain
where
    Chain: CanLogChainPacket<Counterparty>
        + HasIbcPacketTypes<Counterparty>
        + HasReceivePacketPayload<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasAckPacketPayload<Counterparty>
        + HasTimeoutUnorderedPacketPayload<Counterparty>
        + UseExtraChainComponentsForIbcMessageSender<Counterparty>
        + HasComponents<Components = ExtraChainComponents<ChainComponents>>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasUpdateClientPayload<Chain>
        + HasAckPacketPayload<Chain>
        + HasTimeoutUnorderedPacketPayload<Chain>
        + HasReceivePacketPayload<Chain>,
    ChainComponents: PacketFieldsReader<Chain, Counterparty>
        + ReceivedPacketQuerier<Chain, Counterparty>
        + ReceivePacketPayloadBuilder<Chain, Counterparty>
        + ReceivePacketMessageBuilder<Chain, Counterparty>
        + AckPacketPayloadBuilder<Chain, Counterparty>
        + AckPacketMessageBuilder<Chain, Counterparty>
        + TimeoutUnorderedPacketPayloadBuilder<Chain, Counterparty>
        + TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty>,
{
}
