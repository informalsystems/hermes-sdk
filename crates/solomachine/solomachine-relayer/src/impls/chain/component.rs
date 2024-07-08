use cgp_core::prelude::*;
pub use hermes_cosmos_chain_components::components::client::{
    AckPacketPayloadTypeComponent, ChannelOpenAckPayloadTypeComponent,
    ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadTypeComponent,
    ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmPayloadTypeComponent,
    ConnectionOpenInitPayloadTypeComponent, ConnectionOpenTryPayloadTypeComponent,
    CreateClientMessageOptionsTypeComponent, CreateClientPayloadOptionsTypeComponent,
    CreateClientPayloadTypeComponent, InitChannelOptionsTypeComponent,
    InitConnectionOptionsTypeComponent, ReceivePacketPayloadTypeComponent,
    UpdateClientPayloadTypeComponent,
};
pub use hermes_cosmos_chain_components::components::client::{
    ChannelEndTypeComponent, ClientStateFieldsGetterComponent, ClientStateTypeComponent,
    ConsensusStateTypeComponent,
};
pub use hermes_cosmos_chain_components::components::client::{
    ConnectionOpenInitEventComponent, CreateClientEventComponent,
    TimeoutUnorderedPacketPayloadTypeComponent,
};
pub use hermes_cosmos_chain_components::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
pub use hermes_cosmos_chain_components::impls::packet::packet_fields::CosmosPacketFieldReader;
pub use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
pub use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
pub use hermes_encoding_components::traits::has_encoding::EncodingGetterComponent;
pub use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
pub use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
pub use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
pub use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
pub use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
pub use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReaderComponent;
pub use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    ChannelOpenAckPayloadBuilderComponent, ChannelOpenConfirmPayloadBuilderComponent,
    ChannelOpenTryPayloadBuilderComponent,
};
pub use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenConfirmPayloadBuilderComponent,
    ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenTryPayloadBuilderComponent,
};
pub use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
pub use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilderComponent;
pub use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
pub use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
pub use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
pub use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
pub use hermes_relayer_components::chain::traits::types::connection::ConnectionEndTypeComponent;
pub use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
pub use hermes_relayer_components::chain::traits::types::height::{
    HeightFieldComponent, HeightTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
pub use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
pub use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
pub use hermes_relayer_components::chain::traits::types::proof::CommitmentProofTypeComponent;
pub use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
pub use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use crate::impls::chain::solomachine_components::channel_handshake_payload::BuildSolomachineChannelHandshakePayloads;
use crate::impls::chain::solomachine_components::connection_handshake_message::BuildCosmosToSolomachineConnectionHandshakeMessage;
use crate::impls::chain::solomachine_components::connection_handshake_payload::BuildSolomachineConnectionHandshakePayloads;
use crate::impls::chain::solomachine_components::create_client_message::BuildCreateCosmosClientMessage;
use crate::impls::chain::solomachine_components::create_client_payload::BuildSolomachineCreateClientPayload;
use crate::impls::chain::solomachine_components::process_message::ProcessSolomachineMessages;
use crate::impls::chain::solomachine_components::query_chain_status::QuerySolomachineStatus;
use crate::impls::chain::solomachine_components::receive_packet_payload::BuildSolomachineReceivePacketPayload;
use crate::impls::chain::solomachine_components::timeout_packet_payload::BuildSolomachineTimeoutPacketPayload;
use crate::impls::chain::solomachine_components::types::chain::ProvideSolomachineChainTypes;
use crate::impls::chain::solomachine_components::update_client_payload::BuildSolomachineUpdateClientPayload;
use crate::impls::client_state::ProvideSolomachineClientState;
use crate::impls::consensus_state::ProvideSolomachineConsensusState;

define_components! {
    SolomachineChainComponents {
        [
            HeightTypeComponent,
            HeightFieldComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
            CommitmentProofTypeComponent,
            ConnectionEndTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            MessageTypeComponent,
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
        ]:
            ProvideSolomachineChainTypes,
        [
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
        ]:
            ProvideSolomachineClientState,
        ConsensusStateTypeComponent:
            ProvideSolomachineConsensusState,
        PacketFieldsReaderComponent:
            CosmosPacketFieldReader,
        MessageSenderComponent:
            ProcessSolomachineMessages,
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
