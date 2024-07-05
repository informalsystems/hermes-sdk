use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_chain_components::components::client::{
    ClientStateFieldsGetterComponent, ClientStateTypeComponent, ConsensusStateTypeComponent,
};
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
use hermes_cosmos_chain_components::impls::packet::packet_fields::CosmosPacketFieldReader;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::has_encoding::{
    EncodingGetterComponent, HasDefaultEncoding,
};
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::impls::queries::query_and_convert_consensus_state::QueryAndConvertRawConsensusState;
use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReaderComponent;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    ChannelOpenAckPayloadBuilderComponent, ChannelOpenConfirmPayloadBuilderComponent,
    ChannelOpenTryPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    CanBuildConnectionOpenAckPayload, CanBuildConnectionOpenConfirmPayload,
    CanBuildConnectionOpenInitPayload, CanBuildConnectionOpenTryPayload,
    ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenConfirmPayloadBuilderComponent,
    ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenTryPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryClientState, CanQueryClientStateWithProofs, ClientStateQuerierComponent,
    ClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusStateWithProofs, ConsensusStateQuerierComponent,
    ConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::connection::ConnectionEndTypeComponent;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::CreateClientMessageOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    HeightFieldComponent, HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::proof::CommitmentProofTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime_components::traits::runtime::{
    GetRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};

use crate::context::chain::MockSolomachine;
use crate::context::encoding::SolomachineEncoding;
use crate::impls::chain::cosmos_components::connection_handshake_message::BuildSolomachineConnectionHandshakeMessagesForCosmos;
use crate::impls::chain::cosmos_components::create_client_message::BuildCreateSolomachineClientMessage;
use crate::impls::chain::solomachine_components::channel_handshake_payload::BuildSolomachineChannelHandshakePayloads;
use crate::impls::chain::solomachine_components::connection_handshake_message::BuildCosmosToSolomachineConnectionHandshakeMessage;
use crate::impls::chain::solomachine_components::connection_handshake_payload::BuildSolomachineConnectionHandshakePayloads;
use crate::impls::chain::solomachine_components::create_client_message::BuildCreateCosmosClientMessage;
use crate::impls::chain::solomachine_components::create_client_payload::BuildSolomachineCreateClientPayload;
use crate::impls::chain::solomachine_components::process_message::ProcessSolomachineMessages;
use crate::impls::chain::solomachine_components::query_chain_status::QuerySolomachineStatus;
use crate::impls::chain::solomachine_components::query_consensus_state::QueryCosmosConsensusStateFromSolomachine;
use crate::impls::chain::solomachine_components::receive_packet_payload::BuildSolomachineReceivePacketPayload;
use crate::impls::chain::solomachine_components::timeout_packet_payload::BuildSolomachineTimeoutPacketPayload;
use crate::impls::chain::solomachine_components::types::chain::ProvideSolomachineChainTypes;
use crate::impls::chain::solomachine_components::update_client_payload::BuildSolomachineUpdateClientPayload;
use crate::impls::client_state::ProvideSolomachineClientState;
use crate::impls::consensus_state::ProvideSolomachineConsensusState;
use crate::types::consensus_state::SolomachineConsensusState;

pub struct SolomachineChainComponents;

impl HasComponents for MockSolomachine {
    type Components = SolomachineChainComponents;
}

pub struct SolomachineCosmosComponents;

delegate_components! {
    SolomachineCosmosComponents {
        [
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
        ]:
            QueryAndConvertRawClientState,
        [
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
        ]:
            QueryAndConvertRawConsensusState,
        [
            CreateClientMessageBuilderComponent,
            CreateClientMessageOptionsTypeComponent,
        ]:
            BuildCreateSolomachineClientMessage,
        [
            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
        ]:
            BuildSolomachineConnectionHandshakeMessagesForCosmos,
    }
}

impl DelegateComponent<MockSolomachine> for DelegateCosmosChainComponents {
    type Delegate = SolomachineCosmosComponents;
}

delegate_components! {
    SolomachineChainComponents {
        ErrorTypeComponent:
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        RuntimeGetterComponent:
            GetRuntimeField<symbol!("runtime")>,
        [
            HeightTypeComponent,
            HeightFieldComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,
            ConnectionEndTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            MessageTypeComponent,
            EventTypeComponent,
        ]:
            ProvideSolomachineChainTypes,
        [
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
        ]:
            ProvideSolomachineClientState,
        ConsensusStateTypeComponent:
            ProvideSolomachineConsensusState,
        EncodingGetterComponent:
            GetDefaultEncoding,
        PacketFieldsReaderComponent:
            CosmosPacketFieldReader,
        MessageSenderComponent:
            ProcessSolomachineMessages,
        ChainStatusQuerierComponent:
            QuerySolomachineStatus,
        ConsensusStateQuerierComponent:
            QueryCosmosConsensusStateFromSolomachine,
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

pub trait CanUseSolomachine:
    HasConsensusStateType<CosmosChain, ConsensusState = SolomachineConsensusState>
    + HasDefaultEncoding<Encoding = SolomachineEncoding>
{
}

impl CanUseSolomachine for MockSolomachine {}

pub trait CanUseCosmosChainWithSolomachine:
    CanQueryClientState<MockSolomachine>
    + CanQueryClientStateWithProofs<MockSolomachine>
    + CanQueryConsensusStateWithProofs<MockSolomachine>
    + CanBuildConnectionOpenInitPayload<MockSolomachine>
    + CanBuildConnectionOpenTryPayload<MockSolomachine>
    + CanBuildConnectionOpenAckPayload<MockSolomachine>
    + CanBuildConnectionOpenConfirmPayload<MockSolomachine>
{
}

impl CanUseCosmosChainWithSolomachine for CosmosChain {}
