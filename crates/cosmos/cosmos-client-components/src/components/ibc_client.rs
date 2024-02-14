use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;

use crate::impls::client::create_client_message::BuildCosmosCreateClientMessage;
use crate::impls::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use crate::impls::queries::consensus_state::QueryCosmosConsensusStateFromChainHandle;

pub struct CosmosIbcClientComponents;

delegate_components! {
    #[mark_component(IsCosmosIbcClientComponents)]
    CosmosIbcClientComponents {
        CreateClientMessageBuilderComponent:
            BuildCosmosCreateClientMessage,
        ConnectionHandshakeMessageBuilderComponent:
            BuildCosmosConnectionHandshakeMessage,
        ConsensusStateQuerierComponent:
            QueryCosmosConsensusStateFromChainHandle,
    }
}
