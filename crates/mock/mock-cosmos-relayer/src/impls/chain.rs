use std::sync::Arc;
use std::time::Duration;

use basecoin::modules::ibc::AnyConsensusState;
use cgp_core::prelude::*;
use cgp_core::{ErrorRaiser, HasComponents, ProvideErrorType};
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReader;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::AckPacketPayloadBuilder;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilder;
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::TimeoutUnorderedPacketPayloadBuilder;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerier;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerier;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightQuerier;
use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerier;
use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerier;
use hermes_relayer_components::chain::traits::send_message::MessageSender;
use hermes_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, ProvideChainIdType,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientEvent, ProvideCreateClientOptionsType, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::height::{
    HeightIncrementer, ProvideHeightType,
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
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_runtime::types::error::TokioRuntimeError;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc::clients::tendermint::client_state::ClientState as TmClientState;
use ibc::clients::tendermint::consensus_state::ConsensusState as TmConsensusState;
use ibc::clients::tendermint::types::{AllowUpdate, Header, TrustThreshold};
use ibc::clients::tendermint::TENDERMINT_CLIENT_TYPE;
use ibc::core::channel::types::events::{SendPacket, WriteAcknowledgement};
use ibc::core::channel::types::msgs::{MsgAcknowledgement, MsgRecvPacket, MsgTimeout};
use ibc::core::channel::types::packet::Packet;
use ibc::core::channel::types::timeout::TimeoutHeight;
use ibc::core::client::context::ClientValidationContext;
use ibc::core::client::types::events::CreateClient;
use ibc::core::client::types::msgs::{MsgCreateClient, MsgUpdateClient};
use ibc::core::client::types::Height;
use ibc::core::commitment_types::specs::ProofSpecs;
use ibc::core::handler::types::events::IbcEvent;
use ibc::core::host::types::identifiers::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId, Sequence,
};
use ibc::core::host::types::path::{AckPath, ClientConsensusStatePath, ReceiptPath};
use ibc::core::host::ValidationContext;
use ibc::primitives::proto::Any;
use ibc::primitives::{Timestamp, ToProto};

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

impl<Chain: BasecoinEndpoint> ProvideTimestampType<MockCosmosContext<Chain>>
    for MockCosmosChainComponents
{
    type Timestamp = Timestamp;

    fn timestamp_from_nanos(nanos: u64) -> Self::Timestamp {
        Timestamp::from_nanoseconds(nanos).expect("Timestamp::from_nanoseconds is infallible")
    }

    fn timestamp_duration_since(earlier: &Timestamp, later: &Timestamp) -> Option<Duration> {
        later.duration_since(earlier)
    }
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

impl<Chain, Counterparty>
    ProvideClientStateType<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type ClientState = TmClientState;
}

impl<Chain, Counterparty>
    ClientStateFieldsGetter<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    fn client_state_chain_id(client_state: &TmClientState) -> &ChainId {
        &client_state.inner().chain_id
    }

    fn client_state_latest_height(client_state: &TmClientState) -> &Height {
        &client_state.inner().latest_height
    }

    fn client_state_is_frozen(client_state: &TmClientState) -> bool {
        client_state.inner().frozen_height.is_some()
    }

    fn client_state_has_expired(client_state: &TmClientState, elapsed: Duration) -> bool {
        elapsed > client_state.inner().trusting_period
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
        _height: &Height,
    ) -> Result<TmClientState, Error> {
        chain
            .ibc_context()
            .client_state(client_id)
            .map_err(Error::source)
    }
}

impl<Chain, Counterparty>
    ProvideConsensusStateType<MockCosmosContext<Chain>, MockCosmosContext<Counterparty>>
    for MockCosmosChainComponents
where
    Chain: BasecoinEndpoint,
    Counterparty: BasecoinEndpoint,
{
    type ConsensusState = TmConsensusState;
}

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

impl<Chain, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for MockCosmosChainComponents
where
    Chain: Async,
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
        let tm_client_state: TmClientState = ibc::clients::tendermint::types::ClientState::new(
            chain.get_chain_id().clone(),
            TrustThreshold::ONE_THIRD,
            Duration::from_secs(64000),
            Duration::from_secs(128000),
            Duration::from_millis(3000),
            chain.get_current_height(),
            ProofSpecs::cosmos(),
            Vec::new(),
            AllowUpdate {
                after_expiry: false,
                after_misbehaviour: false,
            },
        )
        .map_err(Error::source)?
        .into();

        let current_height = chain.get_current_height();

        let tm_consensus_state = chain
            .ibc_context()
            .host_consensus_state(&current_height)
            .map_err(Error::source)?;

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

impl<Chain, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for MockCosmosChainComponents
where
    Chain: Async,
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

        let default_client_id = ClientId::new(TENDERMINT_CLIENT_TYPE, 0).map_err(Error::source)?;

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
        consensus_height: &Height,
        _query_height: &Height,
    ) -> Result<TmConsensusState, Error> {
        let path = ClientConsensusStatePath::new(
            client_id.clone(),
            consensus_height.revision_number(),
            consensus_height.revision_height(),
        );

        let any_cons_state: AnyConsensusState = chain
            .ibc_context()
            .consensus_state(&path)
            .map_err(Error::source)?;

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
    async fn query_packet_is_received(
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
        let chan_counter = chain
            .ibc_context()
            .channel_counter()
            .map_err(Error::source)?;

        let chan_id = ChannelId::new(chan_counter);

        let port_id = PortId::transfer();

        let ack_path = AckPath::new(&port_id, &chan_id, packet.seq_on_a);

        chain
            .ibc_context()
            .get_packet_acknowledgement(&ack_path)
            .map_err(Error::source)?;

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
