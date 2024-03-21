use cgp_core::HasComponents;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::{
    AckPacketMessageBuilder, CanBuildAckPacketMessage,
};
use hermes_relayer_components::chain::traits::message_builders::receive_packet::{
    CanBuildReceivePacketMessage, ReceivePacketMessageBuilder,
};
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::{
    CanBuildTimeoutUnorderedPacketMessage, TimeoutUnorderedPacketMessageBuilder,
};
use hermes_relayer_components::chain::traits::packet::fields::{
    CanReadPacketFields, PacketFieldsReader,
};
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
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;

use crate::components::extra::chain::DelegatesToExtraChainComponents;
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
    + UseExtraChainComponentsForIbcMessageSender<Counterparty>
where
    Counterparty: HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasIbcChainTypes<Self>
        + HasUpdateClientPayloadType<Self>
        + HasReceivePacketPayloadType<Self>
        + HasAckPacketPayloadType<Self>
        + HasTimeoutUnorderedPacketPayloadType<Self>,
{
}

impl<Chain, Counterparty, Components, BaseComponents>
    UseExtraChainComponentsForPacketRelayer<Counterparty> for Chain
where
    Chain: HasIbcPacketTypes<Counterparty>
        + HasReceivePacketPayloadType<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasAckPacketPayloadType<Counterparty>
        + HasTimeoutUnorderedPacketPayloadType<Counterparty>
        + UseExtraChainComponentsForIbcMessageSender<Counterparty>
        + HasComponents<Components = Components>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasUpdateClientPayloadType<Chain>
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
