use alloc::sync::Arc;

use async_trait::async_trait;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::endpoint::ChainStatus;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_all_in_one::one_for_all::traits::chain::{OfaChain, OfaChainTypes, OfaIbcChain};
use ibc_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerier;
use ibc_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerier;
use ibc_relayer_components::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilder;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerier;
use ibc_relayer_components::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerier;
use ibc_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use ibc_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilder;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSender;
use ibc_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerier;
use ibc_relayer_runtime::types::error::Error as TokioError;
use ibc_relayer_runtime::types::log::logger::TracingLogger;
use ibc_relayer_runtime::types::log::value::LogValue;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_relayer_subscription::traits::subscription::Subscription;
use ibc_relayer_types::core::ics04_channel::events::{SendPacket, WriteAcknowledgement};
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics04_channel::timeout::TimeoutHeight;
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::timestamp::Timestamp;
use ibc_relayer_types::Height;
use prost::Message as _;
use tendermint::abci::Event as AbciEvent;

use crate::contexts::chain::CosmosChain;
use crate::impls::chain::components::connection_handshake_payload::BuildCosmosConnectionHandshakePayload;
use crate::impls::chain::components::create_client_message::BuildCosmosCreateClientMessage;
use crate::impls::chain::components::create_client_payload::BuildCreateClientPayloadWithChainHandle;
use crate::impls::chain::components::query_chain_id::QueryChainIdWithChainHandle;
use crate::impls::chain::components::query_chain_status::QueryChainStatusWithChainHandle;
use crate::impls::chain::components::query_client_state::QueryCosmosClientStateFromChainHandle;
use crate::impls::chain::components::query_consensus_state::QueryCosmosConsensusStateFromChainHandle;
use crate::impls::chain::components::query_write_ack_event::QueryWriteAckEventFromChainHandle;
use crate::impls::chain::components::send_messages_as_tx::SendMessagesToTxContext;
use crate::methods::channel::{
    build_channel_open_ack_message, build_channel_open_ack_payload,
    build_channel_open_confirm_message, build_channel_open_confirm_payload,
    build_channel_open_init_message, build_channel_open_try_message,
    build_channel_open_try_payload,
};
use crate::methods::connection::{
    build_connection_open_ack_message, build_connection_open_confirm_message,
    build_connection_open_init_message, build_connection_open_try_message,
};
use crate::methods::consensus_state::find_consensus_state_height_before;
use crate::methods::event::{
    try_extract_channel_open_init_event, try_extract_channel_open_try_event,
    try_extract_connection_open_init_event, try_extract_connection_open_try_event,
    try_extract_create_client_event, try_extract_send_packet_event, try_extract_write_ack_event,
};
use crate::methods::packet::{
    build_ack_packet_message, build_ack_packet_payload, build_receive_packet_message,
    build_receive_packet_payload, build_timeout_unordered_packet_message,
    build_timeout_unordered_packet_payload, query_is_packet_received,
};
use crate::methods::unreceived_packet::{
    query_packet_commitments, query_send_packets_from_sequences, query_unreceived_packet_sequences,
};
use crate::methods::update_client::{build_update_client_message, build_update_client_payload};
use crate::traits::message::CosmosMessage;
use crate::types::channel::CosmosInitChannelOptions;
use crate::types::connection::CosmosInitConnectionOptions;
use crate::types::error::{BaseError, Error};
use crate::types::events::channel::{CosmosChannelOpenInitEvent, CosmosChannelOpenTryEvent};
use crate::types::events::client::CosmosCreateClientEvent;
use crate::types::events::connection::{
    CosmosConnectionOpenInitEvent, CosmosConnectionOpenTryEvent,
};
use crate::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use crate::types::payloads::client::{CosmosCreateClientPayload, CosmosUpdateClientPayload};
use crate::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use crate::types::payloads::packet::{
    CosmosAckPacketPayload, CosmosReceivePacketPayload, CosmosTimeoutUnorderedPacketPayload,
};
use crate::types::telemetry::CosmosTelemetry;
use crate::types::tendermint::{TendermintClientState, TendermintConsensusState};

#[async_trait]
impl<Chain> OfaChainTypes for CosmosChain<Chain>
where
    Chain: ChainHandle,
{
    type Error = Error;

    type Runtime = TokioRuntimeContext;

    type Logger = TracingLogger;

    type Telemetry = CosmosTelemetry;

    type Message = Arc<dyn CosmosMessage>;

    type Event = Arc<AbciEvent>;

    type ClientState = TendermintClientState;

    type ConsensusState = TendermintConsensusState;

    type Height = Height;

    type Timestamp = Timestamp;

    type ChainId = ChainId;

    type ClientId = ClientId;

    type ConnectionId = ConnectionId;

    type ChannelId = ChannelId;

    type PortId = PortId;

    type Sequence = Sequence;

    type ChainStatus = ChainStatus;

    type IncomingPacket = Packet;

    type OutgoingPacket = Packet;

    type CreateClientPayloadOptions = ClientSettings;

    type InitConnectionOptions = CosmosInitConnectionOptions;

    type InitChannelOptions = CosmosInitChannelOptions;

    type CreateClientPayload = CosmosCreateClientPayload;

    type CreateClientEvent = CosmosCreateClientEvent;

    type UpdateClientPayload = CosmosUpdateClientPayload;

    type ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload;

    type ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload;

    type ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload;

    type ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload;

    type ChannelOpenTryPayload = CosmosChannelOpenTryPayload;

    type ChannelOpenAckPayload = CosmosChannelOpenAckPayload;

    type ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload;

    type ReceivePacketPayload = CosmosReceivePacketPayload;

    type AckPacketPayload = CosmosAckPacketPayload;

    type TimeoutUnorderedPacketPayload = CosmosTimeoutUnorderedPacketPayload;

    type SendPacketEvent = SendPacket;

    type WriteAckEvent = WriteAcknowledgement;

    type ConnectionOpenInitEvent = CosmosConnectionOpenInitEvent;

    type ConnectionOpenTryEvent = CosmosConnectionOpenTryEvent;

    type ChannelOpenInitEvent = CosmosChannelOpenInitEvent;

    type ChannelOpenTryEvent = CosmosChannelOpenTryEvent;
}

#[async_trait]
impl<Chain> OfaChain for CosmosChain<Chain>
where
    Chain: ChainHandle,
{
    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }

    fn runtime_error(e: TokioError) -> Error {
        BaseError::tokio(e).into()
    }

    fn logger(&self) -> &TracingLogger {
        &TracingLogger
    }

    fn log_event(event: &Arc<AbciEvent>) -> LogValue<'_> {
        LogValue::Debug(event)
    }

    fn log_incoming_packet(packet: &Packet) -> LogValue<'_> {
        LogValue::Display(packet)
    }

    fn log_outgoing_packet(packet: &Packet) -> LogValue<'_> {
        LogValue::Display(packet)
    }

    fn telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }

    fn increment_height(height: &Height) -> Result<Height, Error> {
        Ok(height.increment())
    }

    fn estimate_message_size(message: &Arc<dyn CosmosMessage>) -> Result<usize, Error> {
        let raw = message
            .encode_protobuf(&Signer::dummy())
            .map_err(BaseError::encode)?;

        Ok(raw.encoded_len())
    }

    fn chain_status_height(status: &ChainStatus) -> &Height {
        &status.height
    }

    fn chain_status_timestamp(status: &ChainStatus) -> &Timestamp {
        &status.timestamp
    }

    fn try_extract_write_ack_event(event: &Arc<AbciEvent>) -> Option<WriteAcknowledgement> {
        try_extract_write_ack_event(event)
    }

    fn try_extract_send_packet_event(event: &Arc<AbciEvent>) -> Option<SendPacket> {
        try_extract_send_packet_event(event)
    }

    fn extract_packet_from_send_packet_event(event: &SendPacket) -> Packet {
        event.packet.clone()
    }

    fn extract_packet_from_write_ack_event(ack: &WriteAcknowledgement) -> &Packet {
        &ack.packet
    }

    fn try_extract_create_client_event(event: Arc<AbciEvent>) -> Option<CosmosCreateClientEvent> {
        try_extract_create_client_event(event)
    }

    fn create_client_event_client_id(event: &CosmosCreateClientEvent) -> &ClientId {
        &event.client_id
    }

    fn client_state_latest_height(client_state: &TendermintClientState) -> &Height {
        &client_state.latest_height
    }

    fn try_extract_connection_open_init_event(
        event: Arc<AbciEvent>,
    ) -> Option<CosmosConnectionOpenInitEvent> {
        try_extract_connection_open_init_event(event)
    }

    fn connection_open_init_event_connection_id(
        event: &CosmosConnectionOpenInitEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }

    fn try_extract_connection_open_try_event(
        event: Arc<AbciEvent>,
    ) -> Option<CosmosConnectionOpenTryEvent> {
        try_extract_connection_open_try_event(event)
    }

    fn connection_open_try_event_connection_id(
        event: &CosmosConnectionOpenTryEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }

    fn try_extract_channel_open_init_event(
        event: Arc<AbciEvent>,
    ) -> Option<CosmosChannelOpenInitEvent> {
        try_extract_channel_open_init_event(event)
    }

    fn channel_open_try_event_channel_id(event: &CosmosChannelOpenTryEvent) -> &ChannelId {
        &event.channel_id
    }

    fn try_extract_channel_open_try_event(
        event: Arc<AbciEvent>,
    ) -> Option<CosmosChannelOpenTryEvent> {
        try_extract_channel_open_try_event(event)
    }

    fn channel_open_init_event_channel_id(event: &CosmosChannelOpenInitEvent) -> &ChannelId {
        &event.channel_id
    }

    fn chain_id(&self) -> &ChainId {
        &self.chain_id
    }

    async fn send_messages(
        &self,
        messages: Vec<Arc<dyn CosmosMessage>>,
    ) -> Result<Vec<Vec<Arc<AbciEvent>>>, Error> {
        SendMessagesToTxContext::send_messages(self, messages).await
    }

    async fn query_chain_status(&self) -> Result<ChainStatus, Error> {
        QueryChainStatusWithChainHandle::query_chain_status(self).await
    }

    fn event_subscription(&self) -> &Arc<dyn Subscription<Item = (Height, Arc<AbciEvent>)>> {
        &self.subscription
    }

    async fn query_write_ack_event(
        &self,
        packet: &Packet,
    ) -> Result<Option<WriteAcknowledgement>, Error> {
        <QueryWriteAckEventFromChainHandle as WriteAckQuerier<_, ()>>::query_write_ack_event(
            self, packet,
        )
        .await
    }

    async fn build_create_client_payload(
        &self,
        client_settings: &ClientSettings,
    ) -> Result<CosmosCreateClientPayload, Error> {
        <BuildCreateClientPayloadWithChainHandle as CreateClientPayloadBuilder<_, ()>>::build_create_client_payload(self, client_settings).await
    }

    async fn build_update_client_payload(
        &self,
        trusted_height: &Height,
        target_height: &Height,
        client_state: TendermintClientState,
    ) -> Result<CosmosUpdateClientPayload, Error> {
        build_update_client_payload(self, trusted_height, target_height, client_state).await
    }

    async fn build_connection_open_init_payload(
        &self,
        client_state: &TendermintClientState,
    ) -> Result<CosmosConnectionOpenInitPayload, Error> {
        <BuildCosmosConnectionHandshakePayload as ConnectionHandshakePayloadBuilder<Self, ()>>::build_connection_open_init_payload(self, client_state).await
    }

    async fn build_connection_open_try_payload(
        &self,
        client_state: &TendermintClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<CosmosConnectionOpenTryPayload, Error> {
        <BuildCosmosConnectionHandshakePayload as ConnectionHandshakePayloadBuilder<Self, ()>>::build_connection_open_try_payload(self, client_state, height, client_id, connection_id).await
    }

    async fn build_connection_open_ack_payload(
        &self,
        client_state: &TendermintClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<CosmosConnectionOpenAckPayload, Error> {
        <BuildCosmosConnectionHandshakePayload as ConnectionHandshakePayloadBuilder<Self, ()>>::build_connection_open_ack_payload(self, client_state,height, client_id, connection_id).await
    }

    async fn build_connection_open_confirm_payload(
        &self,
        client_state: &TendermintClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<CosmosConnectionOpenConfirmPayload, Error> {
        <BuildCosmosConnectionHandshakePayload as ConnectionHandshakePayloadBuilder<Self, ()>>::build_connection_open_confirm_payload(self, client_state,height, client_id, connection_id).await
    }

    async fn build_channel_open_try_payload(
        &self,
        _client_state: &TendermintClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<CosmosChannelOpenTryPayload, Error> {
        build_channel_open_try_payload(self, height, port_id, channel_id).await
    }

    async fn build_channel_open_ack_payload(
        &self,
        _client_state: &TendermintClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<CosmosChannelOpenAckPayload, Error> {
        build_channel_open_ack_payload(self, height, port_id, channel_id).await
    }

    async fn build_channel_open_confirm_payload(
        &self,
        _client_state: &TendermintClientState,
        height: &Height,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<CosmosChannelOpenConfirmPayload, Error> {
        build_channel_open_confirm_payload(self, height, port_id, channel_id).await
    }

    /// Construct a receive packet to be sent to a destination Cosmos
    /// chain from a source Cosmos chain.
    async fn build_receive_packet_payload(
        &self,
        _client_state: &TendermintClientState,
        height: &Height,
        packet: &Packet,
    ) -> Result<CosmosReceivePacketPayload, Error> {
        build_receive_packet_payload(self, height, packet).await
    }

    /// Construct an acknowledgement packet to be sent from a Cosmos
    /// chain that successfully received a packet from another Cosmos
    /// chain.
    async fn build_ack_packet_payload(
        &self,
        _client_state: &TendermintClientState,
        height: &Height,
        packet: &Packet,
        ack: &WriteAcknowledgement,
    ) -> Result<CosmosAckPacketPayload, Error> {
        build_ack_packet_payload(self, height, packet, ack).await
    }

    /// Construct a timeout packet message to be sent between Cosmos chains
    /// over an unordered channel in the event that a packet that originated
    /// from a source chain was not received.
    async fn build_timeout_unordered_packet_payload(
        &self,
        _client_state: &TendermintClientState,
        height: &Height,
        packet: &Packet,
    ) -> Result<CosmosTimeoutUnorderedPacketPayload, Error> {
        build_timeout_unordered_packet_payload(self, height, packet).await
    }
}

#[async_trait]
impl<Chain, Counterparty> OfaIbcChain<CosmosChain<Counterparty>> for CosmosChain<Chain>
where
    Chain: ChainHandle,
    Counterparty: ChainHandle,
{
    fn incoming_packet_src_channel_id(packet: &Packet) -> &ChannelId {
        &packet.source_channel
    }

    fn incoming_packet_dst_channel_id(packet: &Packet) -> &ChannelId {
        &packet.destination_channel
    }

    fn incoming_packet_src_port(packet: &Packet) -> &PortId {
        &packet.source_port
    }

    fn incoming_packet_dst_port(packet: &Packet) -> &PortId {
        &packet.destination_port
    }

    fn incoming_packet_sequence(packet: &Packet) -> &Sequence {
        &packet.sequence
    }

    fn incoming_packet_timeout_height(packet: &Packet) -> Option<&Height> {
        match &packet.timeout_height {
            TimeoutHeight::Never => None,
            TimeoutHeight::At(h) => Some(h),
        }
    }

    fn incoming_packet_timeout_timestamp(packet: &Packet) -> &Timestamp {
        &packet.timeout_timestamp
    }

    fn outgoing_packet_src_channel_id(packet: &Packet) -> &ChannelId {
        &packet.source_channel
    }

    fn outgoing_packet_dst_channel_id(packet: &Packet) -> &ChannelId {
        &packet.destination_channel
    }

    fn outgoing_packet_src_port(packet: &Packet) -> &PortId {
        &packet.source_port
    }

    fn outgoing_packet_dst_port(packet: &Packet) -> &PortId {
        &packet.destination_port
    }

    fn outgoing_packet_sequence(packet: &Packet) -> &Sequence {
        &packet.sequence
    }

    fn outgoing_packet_timeout_height(packet: &Packet) -> Option<&Height> {
        match &packet.timeout_height {
            TimeoutHeight::Never => None,
            TimeoutHeight::At(h) => Some(h),
        }
    }

    fn outgoing_packet_timeout_timestamp(packet: &Packet) -> &Timestamp {
        &packet.timeout_timestamp
    }

    fn counterparty_message_height_for_update_client(
        message: &Arc<dyn CosmosMessage>,
    ) -> Option<Height> {
        message.counterparty_message_height_for_update_client()
    }

    async fn query_chain_id_from_channel_id(
        &self,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<ChainId, Error> {
        <QueryChainIdWithChainHandle as CounterpartyChainIdQuerier<
            Self,
            CosmosChain<Counterparty>,
        >>::query_chain_id_from_channel_id(self, channel_id, port_id)
        .await
    }

    async fn query_client_state(
        &self,
        client_id: &ClientId,
    ) -> Result<TendermintClientState, Error> {
        <QueryCosmosClientStateFromChainHandle as ClientStateQuerier<
            Self,
            CosmosChain<Counterparty>,
        >>::query_client_state(self, client_id)
        .await
    }

    async fn query_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<TendermintConsensusState, Error> {
        <QueryCosmosConsensusStateFromChainHandle as ConsensusStateQuerier<
            Self,
            CosmosChain<Counterparty>,
        >>::query_consensus_state(self, client_id, height)
        .await
    }

    async fn query_is_packet_received(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<bool, Error> {
        query_is_packet_received(self, port_id, channel_id, sequence).await
    }

    /// Query the sequences of the packets that the chain has committed to be
    /// sent to the counterparty chain, of which the full packet relaying is not
    /// yet completed. Once the chain receives the ack from the counterparty
    /// chain, a given sequence should be removed from the packet commitment list.
    async fn query_packet_commitments(
        &self,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<(Vec<Sequence>, Height), Error> {
        query_packet_commitments(self, channel_id, port_id).await
    }

    /// Given a list of counterparty commitment sequences,
    /// return a filtered list of sequences which the chain
    /// has not received the packet from the counterparty chain.
    async fn query_unreceived_packet_sequences(
        &self,
        channel_id: &ChannelId,
        port_id: &PortId,
        sequences: &[Sequence],
    ) -> Result<Vec<Sequence>, Self::Error> {
        query_unreceived_packet_sequences(self, channel_id, port_id, sequences).await
    }

    /// Given a list of sequences, a channel and port will query a list of outgoing
    /// packets which have not been relayed.
    async fn query_send_packets_from_sequences(
        &self,
        channel_id: &ChannelId,
        port_id: &PortId,
        counterparty_channel_id: &ChannelId,
        counterparty_port_id: &PortId,
        sequences: &[Sequence],
        // The height is given to query the packets from a specific height.
        // This height should be the same as the query height from the
        // `CanQueryPacketCommitments` made on the same chain.
        height: &Height,
    ) -> Result<Vec<Packet>, Self::Error> {
        query_send_packets_from_sequences(
            self,
            channel_id,
            port_id,
            counterparty_channel_id,
            counterparty_port_id,
            sequences,
            height,
        )
        .await
    }

    async fn build_receive_packet_message(
        &self,
        packet: &Packet,
        payload: CosmosReceivePacketPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_receive_packet_message(packet, payload)
    }

    async fn build_ack_packet_message(
        &self,
        packet: &Packet,
        payload: CosmosAckPacketPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_ack_packet_message(packet, payload)
    }

    async fn build_timeout_unordered_packet_message(
        &self,
        packet: &Packet,
        payload: CosmosTimeoutUnorderedPacketPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_timeout_unordered_packet_message(packet, payload)
    }

    async fn build_create_client_message(
        &self,
        payload: CosmosCreateClientPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        <BuildCosmosCreateClientMessage as CreateClientMessageBuilder<
            Self,
            CosmosChain<Counterparty>,
        >>::build_create_client_message(self, payload)
        .await
    }

    async fn build_update_client_message(
        &self,
        client_id: &ClientId,
        payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<Arc<dyn CosmosMessage>>, Error> {
        build_update_client_message(client_id, payload)
    }

    async fn find_consensus_state_height_before(
        &self,
        client_id: &ClientId,
        target_height: &Height,
    ) -> Result<Height, Error> {
        find_consensus_state_height_before(self, client_id, target_height).await
    }

    async fn build_connection_open_init_message(
        &self,
        client_id: &ClientId,
        counterparty_client_id: &ClientId,
        init_connection_options: &CosmosInitConnectionOptions,
        counterparty_payload: CosmosConnectionOpenInitPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_connection_open_init_message(
            self,
            client_id,
            counterparty_client_id,
            init_connection_options,
            counterparty_payload,
        )
        .await
    }

    async fn build_connection_open_try_message(
        &self,
        client_id: &ClientId,
        counterparty_client_id: &ClientId,
        counterparty_connection_id: &ConnectionId,
        counterparty_payload: CosmosConnectionOpenTryPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_connection_open_try_message(
            client_id,
            counterparty_client_id,
            counterparty_connection_id,
            counterparty_payload,
        )
    }

    async fn build_connection_open_ack_message(
        &self,
        connection_id: &ConnectionId,
        counterparty_connection_id: &ConnectionId,
        counterparty_payload: CosmosConnectionOpenAckPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_connection_open_ack_message(
            connection_id,
            counterparty_connection_id,
            counterparty_payload,
        )
    }

    async fn build_connection_open_confirm_message(
        &self,
        connection_id: &ConnectionId,
        counterparty_payload: CosmosConnectionOpenConfirmPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_connection_open_confirm_message(connection_id, counterparty_payload)
    }

    async fn build_channel_open_init_message(
        &self,
        port_id: &PortId,
        counterparty_port_id: &PortId,
        init_channel_options: &CosmosInitChannelOptions,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_channel_open_init_message(port_id, counterparty_port_id, init_channel_options)
    }

    async fn build_channel_open_try_message(
        &self,
        port_id: &PortId,
        counterparty_port_id: &PortId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: CosmosChannelOpenTryPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_channel_open_try_message(
            port_id,
            counterparty_port_id,
            counterparty_channel_id,
            counterparty_payload,
        )
    }

    async fn build_channel_open_ack_message(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: CosmosChannelOpenAckPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_channel_open_ack_message(
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_payload,
        )
    }

    async fn build_channel_open_confirm_message(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty_payload: CosmosChannelOpenConfirmPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        build_channel_open_confirm_message(port_id, channel_id, counterparty_payload)
    }
}
