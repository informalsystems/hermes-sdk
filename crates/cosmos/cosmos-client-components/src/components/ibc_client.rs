use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;

use crate::impls::client::create_client_message::BuildCosmosCreateClientMessage;
use crate::impls::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use crate::impls::query_client_state::QueryCosmosClientStateFromChainHandle;
use crate::impls::query_consensus_state::QueryCosmosConsensusStateFromChainHandle;

pub struct CosmosIbcClientComponents;

delegate_components! {
    #[mark_component(IsCosmosIbcClientComponents)]
    CosmosIbcClientComponents {
        ClientStateQuerierComponent:
            QueryCosmosClientStateFromChainHandle,
        CreateClientMessageBuilderComponent:
            BuildCosmosCreateClientMessage,
        ConnectionHandshakeMessageBuilderComponent:
            BuildCosmosConnectionHandshakeMessage,
        ConsensusStateQuerierComponent:
            QueryCosmosConsensusStateFromChainHandle,
    }
}
