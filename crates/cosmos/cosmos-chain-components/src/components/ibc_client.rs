use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerierComponent;

use crate::impls::client::create_client_message::BuildCosmosCreateClientMessage;
use crate::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
use crate::impls::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use crate::impls::queries::consensus_state::QueryCosmosConsensusStateFromChainHandle;
use crate::impls::queries::consensus_state_height::QueryConsensusStateHeightsFromChainHandle;

pub struct CosmosIbcClientComponents;

delegate_components! {
    #[mark_component(IsCosmosIbcClientComponents)]
    CosmosIbcClientComponents {
        CreateClientMessageBuilderComponent:
            BuildCosmosCreateClientMessage,
        UpdateClientMessageBuilderComponent:
            BuildCosmosUpdateClientMessage,
        ConnectionHandshakeMessageBuilderComponent:
            BuildCosmosConnectionHandshakeMessage,
        ConsensusStateQuerierComponent:
            QueryCosmosConsensusStateFromChainHandle,
        ConsensusStateHeightsQuerierComponent:
            QueryConsensusStateHeightsFromChainHandle,
    }
}
