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
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
    ClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerierComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetterComponent, ClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateFieldComponent, ConsensusStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::CreateClientMessageOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetterComponent;

use crate::impls::channel::channel_handshake_message::BuildCosmosChannelHandshakeMessage;
use crate::impls::client::create_client_message::BuildAnyCreateClientMessage;
use crate::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
use crate::impls::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use crate::impls::message_height::GetCosmosCounterpartyMessageHeight;
use crate::impls::queries::consensus_state_height::QueryConsensusStateHeightsFromGrpc;
use crate::impls::types::client_state::ProvideTendermintClientState;
use crate::impls::types::consensus_state::ProvideTendermintConsensusState;
use crate::impls::types::create_client_options::ProvideNoCreateClientMessageOptionsType;

define_components! {
    CosmosToCosmosComponents {
        [
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
        ]:
            ProvideTendermintClientState,
        [
            ConsensusStateTypeComponent,
            ConsensusStateFieldComponent,
        ]:
            ProvideTendermintConsensusState,
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
