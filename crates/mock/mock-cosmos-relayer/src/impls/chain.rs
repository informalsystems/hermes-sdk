use std::sync::Arc;
use std::time::Duration;

use cgp_core::prelude::*;
use basecoin_app::modules::ibc::AnyConsensusState;
use cgp_core::{ErrorRaiser, HasComponents, ProvideErrorType};
use ibc::clients::ics07_tendermint::client_state::{AllowUpdate, ClientState as TmClientState};
use ibc::clients::ics07_tendermint::consensus_state::ConsensusState as TmConsensusState;
use ibc::clients::ics07_tendermint::header::Header;
use ibc::core::events::IbcEvent;
use ibc::core::ics02_client::events::CreateClient;
use ibc::core::ics02_client::msgs::create_client::MsgCreateClient;
use ibc::core::ics02_client::msgs::update_client::MsgUpdateClient;
use ibc::core::ics04_channel::events::{SendPacket, WriteAcknowledgement};
use ibc::core::ics04_channel::msgs::{MsgAcknowledgement, MsgRecvPacket, MsgTimeout};
use ibc::core::ics04_channel::packet::{Packet, Sequence};
use ibc::core::ics04_channel::timeout::TimeoutHeight;
use ibc::core::ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId};
use ibc::core::ics24_host::path::{AckPath, ClientConsensusStatePath, ReceiptPath};
use ibc::core::timestamp::Timestamp;
use ibc::core::{Msg, ValidationContext};
use ibc::proto::Any;
use ibc::Height;
use hermes_relayer_components::chain::traits::components::ack_packet_message_builder::AckPacketMessageBuilder;
use hermes_relayer_components::chain::traits::components::ack_packet_payload_builder::AckPacketPayloadBuilder;
use hermes_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerier;
use hermes_relayer_components::chain::traits::components::consensus_state_height_querier::ConsensusStateHeightQuerier;
use hermes_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerier;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::components::message_sender::MessageSender;
use hermes_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReader;
use hermes_relayer_components::chain::traits::components::receive_packet_message_builder::ReceivePacketMessageBuilder;
use hermes_relayer_components::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilder;
use hermes_relayer_components::chain::traits::components::received_packet_querier::ReceivedPacketQuerier;
use hermes_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::{
    TimeoutUnorderedPacketMessageBuilder, TimeoutUnorderedPacketPayloadBuilder,
};
use hermes_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerier;
use hermes_relayer_components::chain::traits::logs::event::CanLogChainEvent;
use hermes_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use hermes_relayer_components::chain::traits::types::chain_id::{ChainIdGetter, ProvideChainIdType};
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientEvent, HasCreateClientPayload, ProvideCreateClientOptionsType,
};
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::height::{
    HeightIncrementer, ProvideHeightType
};
use hermes_relayer_components::chain::traits::types::ibc::{
    HasCounterpartyMessageHeight, ProvideIbcChainTypes,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::message::{
    CanEstimateMessageSize, ProvideMessageType,
};
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProvider;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::status::ProvideChainStatusType;
use hermes_relayer_components::chain::traits::types::timestamp::ProvideTimestampType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::error::TokioRuntimeError;
use hermes_relayer_runtime::types::log::value::LogValue;
use hermes_relayer_runtime::types::runtime::HermesRuntime;

use crate::components::chain::MockCosmosChainComponents;
use crate::contexts::chain::MockCosmosContext;
use crate::traits::endpoint::{BasecoinEndpoint, QueryService};
use crate::types::error::Error;
use crate::types::status::ChainStatus;
use crate::util::dummy::dummy_signer;

impl<Chain: BasecoinEndpoint> HasComponents for MockCosmosContext<Chain> {
    type Components = MockCosmosChainComponents;
}

impl<Chain: BasecoinEndpoint> ProvideErrorType<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    type Error = Error;
}

impl<Chain: BasecoinEndpoint> ProvideRuntime<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    fn runtime(chain: &MockCosmosContext<Chain>) -> &HermesRuntime {
        &chain.runtime
    }
}

impl<Chain: BasecoinEndpoint> ErrorRaiser<MockCosmosContext<Chain>, TokioRuntimeError>
    for MockCosmosChainComponents
{
    fn raise_error(e: TokioRuntimeError) -> Error {
        Error::source(e)
    }
}

impl<Chain: BasecoinEndpoint> ProvideChainIdType<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    type ChainId = ChainId;
}

impl<Chain: BasecoinEndpoint> ChainIdGetter<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    fn chain_id(chain: &MockCosmosContext<Chain>) -> &ChainId {
        chain.get_chain_id()
    }
}

impl<Chain: BasecoinEndpoint> ProvideHeightType<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    type Height = Height;
}

impl<Chain: BasecoinEndpoint> ProvideEventType<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    type Event = IbcEvent;
}

impl<Chain: BasecoinEndpoint> CanLogChainEvent for MockCosmosContext<Chain> {
    fn log_event<'a>(event: &Self::Event) -> LogValue<'_> {
        LogValue::Debug(event)
    }
}

impl<Chain: BasecoinEndpoint> ProvideTimestampType<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    type Timestamp = Timestamp;
}

impl<Chain: BasecoinEndpoint> ProvideMessageType<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    type Message = Any;
}

impl<Chain, Counterparty>
    ProvideIbcChainTypes<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type ClientId = ClientId;

    type ConnectionId = ConnectionId;

    type ChannelId = ChannelId;

    type PortId = PortId;

    type Sequence = Sequence;
}

impl<Chain, Counterparty>
    IbcPacketTypesProvider<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type IncomingPacket = Packet;

    type OutgoingPacket = Packet;
}

impl<Chain, Counterparty>
    PacketFieldsReader<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    fn incoming_packet_src_channel_id(packet: &Packet) -> &ChannelId {
        &packet.chan_id_on_a
    }

    fn incoming_packet_src_port(packet: &Packet) -> &PortId {
        &packet.port_id_on_a
    }

    fn incoming_packet_dst_channel_id(packet: &Packet) -> &ChannelId {
        &packet.chan_id_on_b
    }

    fn incoming_packet_dst_port(packet: &Packet) -> &PortId {
        &packet.port_id_on_b
    }

    fn incoming_packet_sequence(packet: &Packet) -> &Sequence {
        &packet.seq_on_a
    }

    fn incoming_packet_timeout_height(packet: &Packet) -> Option<&Height> {
        match &packet.timeout_height_on_b {
            TimeoutHeight::Never => None,
            TimeoutHeight::At(height) => Some(height),
        }
    }

    fn incoming_packet_timeout_timestamp(packet: &Packet) -> &Timestamp {
        &packet.timeout_timestamp_on_b
    }

    fn outgoing_packet_src_channel_id(packet: &Packet) -> &ChannelId {
        &packet.chan_id_on_a
    }

    fn outgoing_packet_src_port(packet: &Packet) -> &PortId {
        &packet.port_id_on_a
    }

    fn outgoing_packet_dst_port(packet: &Packet) -> &PortId {
        &packet.port_id_on_b
    }

    fn outgoing_packet_dst_channel_id(packet: &Packet) -> &ChannelId {
        &packet.chan_id_on_b
    }

    fn outgoing_packet_sequence(packet: &Packet) -> &Sequence {
        &packet.seq_on_a
    }

    fn outgoing_packet_timeout_height(packet: &Packet) -> Option<&Height> {
        match &packet.timeout_height_on_b {
            TimeoutHeight::Never => None,
            TimeoutHeight::At(height) => Some(height),
        }
    }

    fn outgoing_packet_timeout_timestamp(packet: &Packet) -> &Timestamp {
        &packet.timeout_timestamp_on_b
    }
}

impl<Chain, Counterparty> CanLogChainPacket<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    fn log_incoming_packet(packet: &Packet) -> LogValue<'_> {
        LogValue::Display(packet)
    }

    fn log_outgoing_packet(packet: &Packet) -> LogValue<'_> {
        LogValue::Display(packet)
    }
}

impl<Chain, Counterparty> HasClientStateType<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type ClientState = TmClientState;
}

impl<Chain, Counterparty> HasClientStateFields<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    fn client_state_latest_height(client_state: &TmClientState) -> &Self::Height {
        &client_state.latest_height
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ClientStateQuerier<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn query_client_state(
        chain: &MockCosmosContext<Chain>,
        client_id: &ClientId,
    ) -> Result<TmClientState, Error> {
        chain
            .ibc_context()
            .client_state(client_id)
            .map_err(Error::source)
    }
}

impl<Chain, Counterparty> HasConsensusStateType<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type ConsensusState = TmConsensusState;
}

#[async_trait]
impl<Chain, Counterparty>
    ConsensusStateHeightQuerier<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn find_consensus_state_height_before(
        _chain: &MockCosmosContext<Chain>,
        _client_id: &ClientId,
        target_height: &Height,
    ) -> Result<Height, Error> {
        target_height.decrement().map_err(Error::source)
    }
}

impl<Chain: BasecoinEndpoint> ProvideChainStatusType<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    type ChainStatus = ChainStatus;

    fn chain_status_height(status: &Self::ChainStatus) -> &Height {
        &status.height
    }

    fn chain_status_timestamp(status: &Self::ChainStatus) -> &Timestamp {
        &status.timestamp
    }
}

#[async_trait]
impl<Chain: BasecoinEndpoint> ChainStatusQuerier<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    async fn query_chain_status(chain: &MockCosmosContext<Chain>) -> Result<ChainStatus, Error> {
        Ok(ChainStatus::new(
            chain.get_current_height(),
            chain.get_current_timestamp(),
        ))
    }
}

impl<Chain, Counterparty>
    ProvideCreateClientOptionsType<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type CreateClientOptions = ();
}

impl<Chain, Counterparty> HasCreateClientPayload<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type CreateClientPayload = Any;
}

impl<Chain, Counterparty> HasCreateClientEvent<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type CreateClientEvent = CreateClient;

    fn try_extract_create_client_event(event: Self::Event) -> Option<Self::CreateClientEvent> {
        match event {
            IbcEvent::CreateClient(e) => Some(e.clone()),
            _ => None,
        }
    }

    fn create_client_event_client_id(event: &Self::CreateClientEvent) -> &Self::ClientId {
        event.client_id()
    }
}

#[async_trait]
impl<Chain, Counterparty>
    CreateClientPayloadBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_create_client_payload(
        chain: &MockCosmosContext<Chain>,
        _create_client_options: &(),
    ) -> Result<Any, Error> {
        let tm_client_state = TmClientState::new(
            chain.get_chain_id().clone(),
            Default::default(),
            Duration::from_secs(64000),
            Duration::from_secs(128000),
            Duration::from_millis(3000),
            chain.get_current_height(),
            Default::default(),
            Default::default(),
            AllowUpdate {
                after_expiry: false,
                after_misbehaviour: false,
            },
        )?;

        let current_height = chain.get_current_height();

        let any_consensus_state = chain.ibc_context().host_consensus_state(&current_height)?;

        let AnyConsensusState::Tendermint(tm_consensus_state) = any_consensus_state;

        let msg_create_client = MsgCreateClient {
            client_state: tm_client_state.into(),
            consensus_state: tm_consensus_state.into(),
            signer: dummy_signer(),
        };

        Ok(msg_create_client.to_any())
    }
}

#[async_trait]
impl<Chain, Counterparty>
    CreateClientMessageBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_create_client_message(
        _chain: &MockCosmosContext<Chain>,
        counterparty_payload: Any,
    ) -> Result<Any, Error> {
        Ok(counterparty_payload)
    }
}

impl<Chain, Counterparty> HasUpdateClientPayload<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type UpdateClientPayload = MsgUpdateClient;
}

#[async_trait]
impl<Chain, Counterparty>
    UpdateClientPayloadBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_update_client_payload(
        chain: &MockCosmosContext<Chain>,
        trusted_height: &Height,
        target_height: &Height,
        _client_state: TmClientState,
    ) -> Result<MsgUpdateClient, Error> {
        let light_block = chain.get_light_block(target_height)?;

        let header = Header {
            signed_header: light_block.signed_header,
            validator_set: light_block.validators,
            trusted_height: *trusted_height,
            trusted_next_validator_set: light_block.next_validators,
        };

        let default_client_id = ClientId::default();

        let msg_update_client = MsgUpdateClient {
            client_id: default_client_id,
            client_message: header.into(),
            signer: dummy_signer(),
        };

        Ok(msg_update_client)
    }
}

#[async_trait]
impl<Chain, Counterparty>
    UpdateClientMessageBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_update_client_message(
        _chain: &MockCosmosContext<Chain>,
        client_id: &ClientId,
        payload: MsgUpdateClient,
    ) -> Result<Vec<Any>, Error> {
        let mut message = payload;
        message.client_id = client_id.clone();

        Ok(vec![message.to_any()])
    }
}

impl<Chain, Counterparty> HasSendPacketEvent<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type SendPacketEvent = SendPacket;

    fn try_extract_send_packet_event(event: &Self::Event) -> Option<Self::SendPacketEvent> {
        match event {
            IbcEvent::SendPacket(e) => Some(e.clone()),
            _ => None,
        }
    }

    fn extract_packet_from_send_packet_event(
        event: &Self::SendPacketEvent,
    ) -> Self::OutgoingPacket {
        Packet {
            seq_on_a: *event.seq_on_a(),
            port_id_on_a: event.port_id_on_a().clone(),
            chan_id_on_a: event.chan_id_on_a().clone(),
            port_id_on_b: event.port_id_on_b().clone(),
            chan_id_on_b: event.chan_id_on_b().clone(),
            data: event.packet_data().to_vec(),
            timeout_height_on_b: *event.timeout_height_on_b(),
            timeout_timestamp_on_b: *event.timeout_timestamp_on_b(),
        }
    }
}

impl<Chain: BasecoinEndpoint> HeightIncrementer<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    fn increment_height(height: &Height) -> Result<Height, Error> {
        Ok(height.increment())
    }
}

impl<Chain: BasecoinEndpoint> CanEstimateMessageSize for MockCosmosContext<Chain> {
    fn estimate_message_size(_message: &Self::Message) -> Result<usize, Self::Error> {
        // Only single messages are sent by the Mock Chain
        Ok(1)
    }
}

#[async_trait]
impl<Chain: BasecoinEndpoint> MessageSender<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    async fn send_messages(
        chain: &MockCosmosContext<Chain>,
        messages: Vec<Any>,
    ) -> Result<Vec<Vec<IbcEvent>>, Error> {
        chain.submit_messages(messages)
    }
}

impl<Chain, Counterparty> HasCounterpartyMessageHeight<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    fn counterparty_message_height_for_update_client(_message: &Any) -> Option<Height> {
        None
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ConsensusStateQuerier<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn query_consensus_state(
        chain: &MockCosmosContext<Chain>,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<TmConsensusState, Error> {
        let path = ClientConsensusStatePath::new(client_id, height);

        let any_cons_state: AnyConsensusState = chain.ibc_context().consensus_state(&path)?;

        let tm_consensus_state =
            TmConsensusState::try_from(any_cons_state).map_err(Error::source)?;

        Ok(tm_consensus_state)
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ReceivedPacketQuerier<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn query_is_packet_received(
        chain: &MockCosmosContext<Chain>,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<bool, Error> {
        let path = ReceiptPath::new(port_id, channel_id, *sequence);

        let receipt = chain.ibc_context().get_packet_receipt(&path);

        Ok(receipt.is_ok())
    }
}

impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for MockCosmosChainComponents
where
    Chain: Async,
{
    type ReceivePacketPayload = Any;
}

#[async_trait]
impl<Chain, Counterparty>
    ReceivePacketPayloadBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_receive_packet_payload(
        chain: &MockCosmosContext<Chain>,
        _client_state: &TmClientState,
        height: &Height,
        packet: &Packet,
    ) -> Result<Any, Error> {
        let receipt_path =
            ReceiptPath::new(&packet.port_id_on_a, &packet.chan_id_on_a, packet.seq_on_a);

        let (_, proof_commitment_on_a) = chain.query(receipt_path, height).await?;

        let recv_packet_payload = MsgRecvPacket {
            packet: packet.clone(),
            proof_commitment_on_a,
            proof_height_on_a: *height,
            signer: dummy_signer(),
        };

        Ok(recv_packet_payload.to_any())
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ReceivePacketMessageBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_receive_packet_message(
        _chain: &MockCosmosContext<Chain>,
        _packet: &Packet,
        payload: Any,
    ) -> Result<Any, Error> {
        Ok(payload)
    }
}

#[async_trait]
impl<Chain, Counterparty> WriteAckQuerier<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn query_write_ack_event(
        chain: &MockCosmosContext<Chain>,
        packet: &Packet,
    ) -> Result<Option<WriteAcknowledgement>, Error> {
        let chan_counter = chain.ibc_context().channel_counter()?;

        let chan_id = ChannelId::new(chan_counter);

        let port_id = PortId::transfer();

        let ack_path = AckPath::new(&port_id, &chan_id, packet.seq_on_a);

        chain.ibc_context().get_packet_acknowledgement(&ack_path)?;

        let events = chain.ibc_context().events();

        for e in events {
            if let IbcEvent::WriteAcknowledgement(e) = e {
                if e.port_id_on_a() == &port_id
                    && e.chan_id_on_a() == &chan_id
                    && e.seq_on_a() == &packet.seq_on_a
                {
                    return Ok(Some(e));
                }
            }
        }

        Ok(None)
    }
}

impl<Chain, Counterparty> HasWriteAckEvent<MockCosmosContext<Counterparty>>
    for MockCosmosContext<Chain>
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type WriteAckEvent = WriteAcknowledgement;

    fn try_extract_write_ack_event(event: &Self::Event) -> Option<Self::WriteAckEvent> {
        match event {
            IbcEvent::WriteAcknowledgement(e) => Some(e.clone()),
            _ => None,
        }
    }
}

impl<Chain, Counterparty> ProvideAckPacketPayloadType<Chain, Counterparty>
    for MockCosmosChainComponents
where
    Chain: Async,
{
    type AckPacketPayload = Any;
}

#[async_trait]
impl<Chain, Counterparty>
    AckPacketPayloadBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_ack_packet_payload(
        chain: &MockCosmosContext<Chain>,
        _client_state: &TmClientState,
        height: &Height,
        packet: &Packet,
        ack: &WriteAcknowledgement,
    ) -> Result<Any, Error> {
        let ack_path = AckPath::new(&packet.port_id_on_a, &packet.chan_id_on_a, packet.seq_on_a);

        let (_, proof_acked_on_b) = chain.query(ack_path, height).await?;

        let ack_packet_payload = MsgAcknowledgement {
            packet: packet.clone(),
            acknowledgement: ack.acknowledgement().clone(),
            proof_acked_on_b,
            proof_height_on_b: chain.get_current_height(),
            signer: dummy_signer(),
        };

        Ok(ack_packet_payload.to_any())
    }
}

#[async_trait]
impl<Chain, Counterparty>
    AckPacketMessageBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_ack_packet_message(
        _chain: &MockCosmosContext<Chain>,
        _packet: &Packet,
        payload: Any,
    ) -> Result<Any, Error> {
        Ok(payload)
    }
}

impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for MockCosmosChainComponents
where
    Chain: Async,
{
    type TimeoutUnorderedPacketPayload = Any;
}

#[async_trait]
impl<Chain, Counterparty>
    TimeoutUnorderedPacketPayloadBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_timeout_unordered_packet_payload(
        chain: &MockCosmosContext<Chain>,
        _client_state: &TmClientState,
        height: &Height,
        packet: &Packet,
    ) -> Result<Any, Error> {
        let receipt_path =
            ReceiptPath::new(&packet.port_id_on_a, &packet.chan_id_on_a, packet.seq_on_a);

        let (_, proof_acked_on_b) = chain.query(receipt_path, height).await?;

        let ack_packet_payload = MsgTimeout {
            packet: packet.clone(),
            next_seq_recv_on_b: packet.seq_on_a.increment(),
            proof_unreceived_on_b: proof_acked_on_b,
            proof_height_on_b: chain.get_current_height(),
            signer: dummy_signer(),
        };

        Ok(ack_packet_payload.to_any())
    }
}

#[async_trait]
impl<Chain, Counterparty>
    TimeoutUnorderedPacketMessageBuilder<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    async fn build_timeout_unordered_packet_message(
        _chain: &MockCosmosContext<Chain>,
        _packet: &Packet,
        payload: Any,
    ) -> Result<Any, Error> {
        Ok(payload)
    }
}

impl<Endpoint> QueryService for MockCosmosContext<Endpoint>
where
    Endpoint: BasecoinEndpoint,
{
    type Endpoint = Endpoint;

    fn service(&self) -> &Arc<Self::Endpoint> {
        &self.querier
    }
}
