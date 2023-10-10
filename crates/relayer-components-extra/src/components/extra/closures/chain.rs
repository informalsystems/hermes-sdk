use cgp_core::HasComponents;
use ibc_relayer_components::chain::traits::components::ack_packet_message_builder::{
    AckPacketMessageBuilder, CanBuildAckPacketMessage,
};
use ibc_relayer_components::chain::traits::components::ack_packet_payload_builder::{
    AckPacketPayloadBuilder, CanBuildAckPacketPayload,
};
use ibc_relayer_components::chain::traits::components::chain_status_querier::{
    CanQueryChainStatus, ChainStatusQuerier,
};
use ibc_relayer_components::chain::traits::components::client_state_querier::{
    CanQueryClientState, ClientStateQuerier,
};
use ibc_relayer_components::chain::traits::components::consensus_state_height_querier::{
    CanQueryConsensusStateHeight, ConsensusStateHeightQuerier,
};
use ibc_relayer_components::chain::traits::components::consensus_state_querier::{
    CanQueryConsensusState, ConsensusStateQuerier,
};
use ibc_relayer_components::chain::traits::components::counterparty_chain_id_querier::{
    CanQueryCounterpartyChainId, CounterpartyChainIdQuerier,
};
use ibc_relayer_components::chain::traits::components::message_sender::{
    CanSendMessages, MessageSender,
};
use ibc_relayer_components::chain::traits::components::packet_fields_reader::{
    CanReadPacketFields, PacketFieldsReader,
};
use ibc_relayer_components::chain::traits::components::packet_from_write_ack_builder::{
    CanBuildPacketFromWriteAck, PacketFromWriteAckBuilder,
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
use ibc_relayer_components::chain::traits::components::update_client_message_builder::{
    CanBuildUpdateClientMessage, UpdateClientMessageBuilder,
};
use ibc_relayer_components::chain::traits::components::update_client_payload_builder::{
    CanBuildUpdateClientPayload, UpdateClientPayloadBuilder,
};
use ibc_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::height::{CanIncrementHeight, HasHeightType};
use ibc_relayer_components::chain::traits::types::ibc::{
    HasCounterpartyMessageHeight, HasIbcChainTypes,
};
use ibc_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use ibc_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;
use ibc_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayload;
use ibc_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use ibc_relayer_components::chain::traits::types::status::HasChainStatusType;
use ibc_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use ibc_relayer_components::logger::traits::has_logger::HasLoggerType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::components::extra::chain::ExtraChainComponents;
use crate::telemetry::traits::metrics::HasBasicMetrics;
use crate::telemetry::traits::telemetry::HasTelemetry;

pub trait CanUseExtraChainComponents<Counterparty>: UseExtraChainComponents<Counterparty>
where
    Counterparty: HasHeightType
        + HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasIbcChainTypes<Self>
        + HasUpdateClientPayload<Self>
        + HasReceivePacketPayload<Self>
        + HasAckPacketPayload<Self>
        + HasTimeoutUnorderedPacketPayload<Self>,
{
}

pub trait UseExtraChainComponents<Counterparty>:
    HasRuntime
    + HasChainId
    + HasLoggerType
    + CanIncrementHeight
    + CanSendMessages
    + CanQueryChainStatus
    + HasConsensusStateType<Counterparty>
    + HasClientStateFields<Counterparty>
    + HasSendPacketEvent<Counterparty>
    + HasCounterpartyMessageHeight<Counterparty>
    + CanLogChainPacket<Counterparty>
    + CanQueryClientState<Counterparty>
    + CanQueryConsensusState<Counterparty>
    + CanQueryConsensusStateHeight<Counterparty>
    + CanQueryReceivedPacket<Counterparty>
    + CanQueryCounterpartyChainId<Counterparty>
    + CanReadPacketFields<Counterparty>
    + CanBuildUpdateClientPayload<Counterparty>
    + CanBuildUpdateClientMessage<Counterparty>
    + CanBuildReceivePacketPayload<Counterparty>
    + CanBuildReceivePacketMessage<Counterparty>
    + CanBuildAckPacketPayload<Counterparty>
    + CanBuildAckPacketMessage<Counterparty>
    + CanBuildTimeoutUnorderedPacketPayload<Counterparty>
    + CanBuildTimeoutUnorderedPacketMessage<Counterparty>
    + CanBuildPacketFromWriteAck<Counterparty>
where
    Counterparty: HasHeightType
        + HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasIbcChainTypes<Self>
        + HasUpdateClientPayload<Self>
        + HasReceivePacketPayload<Self>
        + HasAckPacketPayload<Self>
        + HasTimeoutUnorderedPacketPayload<Self>,
{
}

impl<Chain, Counterparty, ChainComponents> UseExtraChainComponents<Counterparty> for Chain
where
    Chain: HasRuntime
        + HasChainId
        + HasLoggerType
        + CanIncrementHeight
        + HasTelemetry
        + HasChainStatusType
        + HasConsensusStateType<Counterparty>
        + HasClientStateFields<Counterparty>
        + HasSendPacketEvent<Counterparty>
        + HasCounterpartyMessageHeight<Counterparty>
        + CanLogChainPacket<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasClientStateType<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasIbcPacketTypes<Counterparty>
        + HasUpdateClientPayload<Counterparty>
        + HasReceivePacketPayload<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasAckPacketPayload<Counterparty>
        + HasTimeoutUnorderedPacketPayload<Counterparty>
        + HasComponents<Components = ExtraChainComponents<ChainComponents>>,
    Counterparty: HasHeightType
        + HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasUpdateClientPayload<Chain>
        + HasAckPacketPayload<Chain>
        + HasTimeoutUnorderedPacketPayload<Chain>
        + HasReceivePacketPayload<Chain>,
    Chain::Telemetry: HasBasicMetrics,
    ChainComponents: MessageSender<Chain>
        + ChainStatusQuerier<Chain>
        + ConsensusStateQuerier<Chain, Counterparty>
        + ClientStateQuerier<Chain, Counterparty>
        + PacketFieldsReader<Chain, Counterparty>
        + ConsensusStateHeightQuerier<Chain, Counterparty>
        + ReceivedPacketQuerier<Chain, Counterparty>
        + UpdateClientPayloadBuilder<Chain, Counterparty>
        + UpdateClientMessageBuilder<Chain, Counterparty>
        + ReceivePacketPayloadBuilder<Chain, Counterparty>
        + ReceivePacketMessageBuilder<Chain, Counterparty>
        + AckPacketPayloadBuilder<Chain, Counterparty>
        + AckPacketMessageBuilder<Chain, Counterparty>
        + TimeoutUnorderedPacketPayloadBuilder<Chain, Counterparty>
        + TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty>
        + CounterpartyChainIdQuerier<Chain, Counterparty>
        + PacketFromWriteAckBuilder<Chain, Counterparty>,
{
}
