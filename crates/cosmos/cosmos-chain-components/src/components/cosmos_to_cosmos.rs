use cgp::prelude::*;
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::impls::queries::query_and_convert_consensus_state::QueryAndConvertRawConsensusState;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilderComponent, ChannelOpenConfirmMessageBuilderComponent,
    ChannelOpenInitMessageBuilderComponent, ChannelOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
    ClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerierComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsComponent, ClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateFieldComponent, ConsensusStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsTypeComponent, CreateClientPayloadOptionsTypeComponent,
    CreateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetterComponent;
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;

use crate::impls::channel::channel_handshake_message::BuildCosmosChannelHandshakeMessage;
use crate::impls::client::create_client_message::BuildAnyCreateClientMessage;
use crate::impls::client::create_client_payload::BuildCosmosCreateClientPayload;
use crate::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
use crate::impls::client::update_client_payload::BuildTendermintUpdateClientPayload;
use crate::impls::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use crate::impls::message_height::GetCosmosCounterpartyMessageHeight;
use crate::impls::queries::consensus_state_height::QueryConsensusStateHeightsFromGrpc;
use crate::impls::types::client_state::ProvideTendermintClientState;
use crate::impls::types::consensus_state::ProvideTendermintConsensusState;
use crate::impls::types::create_client_options::{
    ProvideCosmosCreateClientSettings, ProvideNoCreateClientMessageOptionsType,
};
use crate::impls::types::payload::ProvideCosmosPayloadTypes;

define_components! {
    CosmosToCosmosComponents {
        [
            ClientStateTypeComponent,
            ClientStateFieldsComponent,
        ]:
            ProvideTendermintClientState,
        [
            ConsensusStateTypeComponent,
            ConsensusStateFieldComponent,
        ]:
            ProvideTendermintConsensusState,
        [
            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
        ]:
            ProvideCosmosPayloadTypes,
        CreateClientPayloadOptionsTypeComponent:
            ProvideCosmosCreateClientSettings,
        [
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
            AllClientStatesQuerierComponent,
        ]:
            QueryAndConvertRawClientState,
        [
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
        ]:
            QueryAndConvertRawConsensusState,
        CreateClientMessageOptionsTypeComponent:
            ProvideNoCreateClientMessageOptionsType,
        CreateClientMessageBuilderComponent:
            BuildAnyCreateClientMessage,
        UpdateClientMessageBuilderComponent:
            BuildCosmosUpdateClientMessage,
        CreateClientPayloadBuilderComponent:
            BuildCosmosCreateClientPayload,
        UpdateClientPayloadBuilderComponent:
            BuildTendermintUpdateClientPayload,
        [
            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
        ]:
            BuildCosmosConnectionHandshakeMessage,
        [
            ChannelOpenInitMessageBuilderComponent,
            ChannelOpenTryMessageBuilderComponent,
            ChannelOpenAckMessageBuilderComponent,
            ChannelOpenConfirmMessageBuilderComponent,
        ]:
            BuildCosmosChannelHandshakeMessage,
        ConsensusStateHeightsQuerierComponent:
            QueryConsensusStateHeightsFromGrpc,
        CounterpartyMessageHeightGetterComponent:
            GetCosmosCounterpartyMessageHeight,
    }
}
