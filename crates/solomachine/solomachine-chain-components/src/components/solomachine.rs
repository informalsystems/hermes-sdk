#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_cosmos_chain_components::components::client::{
        AckPacketPayloadTypeComponent, ChannelEndTypeComponent, ChannelIdTypeComponent,
        ChannelOpenAckPayloadTypeComponent, ChannelOpenConfirmPayloadTypeComponent,
        ChannelOpenTryPayloadTypeComponent, ClientIdTypeComponent, ClientStateFieldsComponent,
        ClientStateTypeComponent, ConnectionIdTypeComponent, ConnectionOpenAckPayloadTypeComponent,
        ConnectionOpenConfirmPayloadTypeComponent, ConnectionOpenInitEventComponent,
        ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent,
        ConsensusStateTypeComponent, CreateClientEventComponent,
        CreateClientMessageOptionsTypeComponent, CreateClientPayloadOptionsTypeComponent,
        CreateClientPayloadTypeComponent, InitChannelOptionsTypeComponent,
        InitConnectionOptionsTypeComponent, MessageResponseEventsGetterComponent,
        MessageResponseTypeComponent, OutgoingPacketTypeComponent, PortIdTypeComponent,
        ReceivePacketPayloadTypeComponent, SequenceTypeComponent, TimeTypeComponent,
        TimeoutUnorderedPacketPayloadTypeComponent, UpdateClientPayloadTypeComponent,
    };
    use hermes_cosmos_chain_components::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
    use hermes_cosmos_chain_components::impls::packet::packet_fields::CosmosPacketFieldReader;
    use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
    use hermes_cosmos_relayer::presets::chain::{
        EventExtractorComponent, ExtractFromMessageResponseViaEvents,
        MessageResponseExtractorComponent, PacketDstChannelIdGetterComponent,
        PacketDstPortIdGetterComponent, PacketSequenceGetterComponent,
        PacketSrcChannelIdGetterComponent, PacketSrcPortIdGetterComponent,
        PacketTimeoutHeightGetterComponent, PacketTimeoutTimestampGetterComponent,
    };
    use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
    use hermes_encoding_components::traits::has_encoding::EncodingGetterComponent;
    use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
    use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
        ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
        ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
    };
    use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
    use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
    use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
    use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
        ChannelOpenAckPayloadBuilderComponent, ChannelOpenConfirmPayloadBuilderComponent,
        ChannelOpenTryPayloadBuilderComponent,
    };
    use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
        ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenConfirmPayloadBuilderComponent,
        ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenTryPayloadBuilderComponent,
    };
    use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
    use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilderComponent;
    use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
    use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
    use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
    use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
    use hermes_relayer_components::chain::traits::types::connection::ConnectionEndTypeComponent;
    use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
    use hermes_relayer_components::chain::traits::types::height::{
        HeightFieldComponent, HeightTypeComponent,
    };
    use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
    use hermes_relayer_components::chain::traits::types::proof::CommitmentProofTypeComponent;
    use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
    use hermes_relayer_components::chain::traits::types::timestamp::TimeoutTypeComponent;

    use crate::impls::solomachine::channel_handshake_payload::BuildSolomachineChannelHandshakePayloads;
    use crate::impls::solomachine::client_state::ProvideSolomachineClientState;
    use crate::impls::solomachine::connection_handshake_message::BuildCosmosToSolomachineConnectionHandshakeMessage;
    use crate::impls::solomachine::connection_handshake_payload::BuildSolomachineConnectionHandshakePayloads;
    use crate::impls::solomachine::consensus_state::ProvideSolomachineConsensusState;
    use crate::impls::solomachine::create_client_message::BuildCreateCosmosClientMessage;
    use crate::impls::solomachine::create_client_payload::BuildSolomachineCreateClientPayload;
    use crate::impls::solomachine::process_message::ProcessSolomachineMessages;
    use crate::impls::solomachine::query_chain_status::QuerySolomachineStatus;
    use crate::impls::solomachine::receive_packet_payload::BuildSolomachineReceivePacketPayload;
    use crate::impls::solomachine::timeout_packet_payload::BuildSolomachineTimeoutPacketPayload;
    use crate::impls::solomachine::types::ProvideSolomachineChainTypes;
    use crate::impls::solomachine::update_client_payload::BuildSolomachineUpdateClientPayload;

    cgp_preset! {
        SolomachineChainComponents {
            [
                HeightTypeComponent,
                HeightFieldComponent,
                TimeTypeComponent,
                TimeoutTypeComponent,
                ChainIdTypeComponent,
                ClientIdTypeComponent,
                ConnectionIdTypeComponent,
                ChannelIdTypeComponent,
                PortIdTypeComponent,
                SequenceTypeComponent,
                OutgoingPacketTypeComponent,
                ChainStatusTypeComponent,
                CommitmentProofTypeComponent,
                ConnectionEndTypeComponent,
            ]:
                ProvideCosmosChainTypes,
            [
                MessageTypeComponent,
                MessageResponseTypeComponent,
                MessageResponseEventsGetterComponent,
                EventTypeComponent,
                ChannelEndTypeComponent,
                CommitmentPrefixTypeComponent,
                CreateClientPayloadOptionsTypeComponent,
                CreateClientMessageOptionsTypeComponent,
                CreateClientPayloadTypeComponent,
                UpdateClientPayloadTypeComponent,
                InitConnectionOptionsTypeComponent,
                InitChannelOptionsTypeComponent,
                ConnectionOpenInitPayloadTypeComponent,
                ConnectionOpenTryPayloadTypeComponent,
                ConnectionOpenAckPayloadTypeComponent,
                ConnectionOpenConfirmPayloadTypeComponent,
                ChannelOpenTryPayloadTypeComponent,
                ChannelOpenAckPayloadTypeComponent,
                ChannelOpenConfirmPayloadTypeComponent,
                ReceivePacketPayloadTypeComponent,
                AckPacketPayloadTypeComponent,
                TimeoutUnorderedPacketPayloadTypeComponent,
                CreateClientEventComponent,
                ConnectionOpenInitEventComponent,
                EventExtractorComponent,
            ]:
                ProvideSolomachineChainTypes,
            [
                ClientStateTypeComponent,
                ClientStateFieldsComponent,
            ]:
                ProvideSolomachineClientState,
            ConsensusStateTypeComponent:
                ProvideSolomachineConsensusState,
            [
                PacketSrcChannelIdGetterComponent,
                PacketSrcPortIdGetterComponent,
                PacketDstChannelIdGetterComponent,
                PacketDstPortIdGetterComponent,
                PacketSequenceGetterComponent,
                PacketTimeoutHeightGetterComponent,
                PacketTimeoutTimestampGetterComponent,
            ]:
                CosmosPacketFieldReader,
            MessageSenderComponent:
                ProcessSolomachineMessages,
            MessageResponseExtractorComponent:
                ExtractFromMessageResponseViaEvents,
            ChainStatusQuerierComponent:
                QuerySolomachineStatus,
            [
                ChannelOpenTryPayloadBuilderComponent,
                ChannelOpenAckPayloadBuilderComponent,
                ChannelOpenConfirmPayloadBuilderComponent,
            ]:
                BuildSolomachineChannelHandshakePayloads,
            [
                ConnectionOpenInitPayloadBuilderComponent,
                ConnectionOpenTryPayloadBuilderComponent,
                ConnectionOpenAckPayloadBuilderComponent,
                ConnectionOpenConfirmPayloadBuilderComponent,
            ]:
                BuildSolomachineConnectionHandshakePayloads,

            [
                ConnectionOpenInitMessageBuilderComponent,
                ConnectionOpenTryMessageBuilderComponent,
                ConnectionOpenAckMessageBuilderComponent,
                ConnectionOpenConfirmMessageBuilderComponent,
            ]:
                BuildCosmosToSolomachineConnectionHandshakeMessage,

            CreateClientPayloadBuilderComponent:
                BuildSolomachineCreateClientPayload,
            CreateClientMessageBuilderComponent:
                BuildCreateCosmosClientMessage,
            ReceivePacketPayloadBuilderComponent:
                BuildSolomachineReceivePacketPayload,
            TimeoutUnorderedPacketMessageBuilderComponent:
                BuildSolomachineTimeoutPacketPayload,
            UpdateClientPayloadBuilderComponent:
                BuildSolomachineUpdateClientPayload,
            UpdateClientMessageBuilderComponent:
                BuildCosmosUpdateClientMessage,
        }
    }
}
