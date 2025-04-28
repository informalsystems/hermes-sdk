#[cgp::re_export_imports]
mod preset {
    use hermes_chain_type_components::traits::{
        MessageResponseEventsGetterComponent, MessageResponseTypeComponent, TimeTypeComponent,
    };
    use hermes_cosmos_chain_components::impls::{
        BuildCosmosUpdateClientMessage, CosmosPacketFieldReader, ProvideCosmosChainTypes,
    };
    use hermes_prelude::*;
    use hermes_relayer_components::chain::traits::{
        AckPacketPayloadTypeProviderComponent, ChainIdTypeProviderComponent,
        ChainStatusQuerierComponent, ChainStatusTypeComponent, ChannelEndTypeComponent,
        ChannelIdTypeComponent, ChannelOpenAckPayloadBuilderComponent,
        ChannelOpenAckPayloadTypeComponent, ChannelOpenConfirmPayloadBuilderComponent,
        ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadBuilderComponent,
        ChannelOpenTryPayloadTypeComponent, ClientIdTypeComponent, ClientStateFieldsComponent,
        ClientStateTypeComponent, CommitmentPrefixTypeComponent,
        CommitmentProofTypeProviderComponent, ConnectionEndTypeComponent,
        ConnectionIdTypeComponent, ConnectionOpenAckMessageBuilderComponent,
        ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenAckPayloadTypeComponent,
        ConnectionOpenConfirmMessageBuilderComponent, ConnectionOpenConfirmPayloadBuilderComponent,
        ConnectionOpenConfirmPayloadTypeComponent, ConnectionOpenInitEventComponent,
        ConnectionOpenInitMessageBuilderComponent, ConnectionOpenInitPayloadBuilderComponent,
        ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryMessageBuilderComponent,
        ConnectionOpenTryPayloadBuilderComponent, ConnectionOpenTryPayloadTypeComponent,
        ConsensusStateTypeComponent, CreateClientEventComponent,
        CreateClientMessageBuilderComponent, CreateClientMessageOptionsTypeComponent,
        CreateClientPayloadBuilderComponent, CreateClientPayloadOptionsTypeComponent,
        CreateClientPayloadTypeComponent, EventExtractorComponent, EventTypeProviderComponent,
        ExtractFromMessageResponseViaEvents, HeightFieldComponent, HeightTypeProviderComponent,
        InitChannelOptionsTypeComponent, InitConnectionOptionsTypeComponent,
        MessageResponseExtractorComponent, MessageSenderComponent, MessageTypeProviderComponent,
        OutgoingPacketTypeComponent, PacketDstChannelIdGetterComponent,
        PacketDstPortIdGetterComponent, PacketSequenceGetterComponent,
        PacketSrcChannelIdGetterComponent, PacketSrcPortIdGetterComponent,
        PacketTimeoutHeightGetterComponent, PacketTimeoutTimestampGetterComponent,
        PortIdTypeComponent, ReceivePacketPayloadBuilderComponent,
        ReceivePacketPayloadTypeComponent, SequenceTypeComponent, TimeoutTypeComponent,
        TimeoutUnorderedPacketMessageBuilderComponent, TimeoutUnorderedPacketPayloadTypeComponent,
        UpdateClientMessageBuilderComponent, UpdateClientPayloadBuilderComponent,
        UpdateClientPayloadTypeComponent,
    };

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
                HeightTypeProviderComponent,
                HeightFieldComponent,
                TimeTypeComponent,
                TimeoutTypeComponent,
                ChainIdTypeProviderComponent,
                ClientIdTypeComponent,
                ConnectionIdTypeComponent,
                ChannelIdTypeComponent,
                PortIdTypeComponent,
                SequenceTypeComponent,
                OutgoingPacketTypeComponent,
                ChainStatusTypeComponent,
                CommitmentProofTypeProviderComponent,
                ConnectionEndTypeComponent,
            ]:
                ProvideCosmosChainTypes,
            [
                MessageTypeProviderComponent,
                MessageResponseTypeComponent,
                MessageResponseEventsGetterComponent,
                EventTypeProviderComponent,
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
                AckPacketPayloadTypeProviderComponent,
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
