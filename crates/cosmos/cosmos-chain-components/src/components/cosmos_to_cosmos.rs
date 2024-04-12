use cgp_core::prelude::*;
use hermes_relayer_components::chain::impls::queries::query_and_decode_client_state::QueryAndDecodeClientStateVia;
use hermes_relayer_components::chain::impls::queries::query_and_decode_consensus_state::QueryAndDecodeConsensusStateVia;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;
use prost_types::Any;

use crate::impls::client::create_client_message::BuildCosmosCreateClientMessage;
use crate::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
use crate::impls::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;

pub struct CosmosToCosmosComponents;

delegate_components! {
    CosmosToCosmosComponents {
        [
            ClientStateQuerierComponent,
            AllClientStatesQuerierComponent,
        ]:
            QueryAndDecodeClientStateVia<Any>,
        ConsensusStateQuerierComponent:
            QueryAndDecodeConsensusStateVia<Any>,
        CreateClientMessageBuilderComponent:
            BuildCosmosCreateClientMessage,
        UpdateClientMessageBuilderComponent:
            BuildCosmosUpdateClientMessage,
        ConnectionHandshakeMessageBuilderComponent:
            BuildCosmosConnectionHandshakeMessage,
    }
}
