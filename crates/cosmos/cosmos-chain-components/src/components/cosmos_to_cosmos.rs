use cgp_core::prelude::*;
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::impls::queries::query_and_convert_consensus_state::QueryAndConvertRawConsensusState;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerierComponent;

use crate::impls::client::create_client_message::BuildCosmosCreateClientMessage;
use crate::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
use crate::impls::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use crate::impls::queries::consensus_state_height::QueryConsensusStateHeightsFromGrpc;

pub struct CosmosToCosmosComponents;

delegate_components! {
    CosmosToCosmosComponents {
        [
            ClientStateQuerierComponent,
            AllClientStatesQuerierComponent,
        ]:
            QueryAndConvertRawClientState,
        ConsensusStateQuerierComponent:
            QueryAndConvertRawConsensusState,
        CreateClientMessageBuilderComponent:
            BuildCosmosCreateClientMessage,
        UpdateClientMessageBuilderComponent:
            BuildCosmosUpdateClientMessage,
        ConnectionHandshakeMessageBuilderComponent:
            BuildCosmosConnectionHandshakeMessage,
        ConsensusStateHeightsQuerierComponent:
            QueryConsensusStateHeightsFromGrpc,
    }
}
