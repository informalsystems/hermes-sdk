use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerierComponent;
use hermes_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilderComponent;

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
