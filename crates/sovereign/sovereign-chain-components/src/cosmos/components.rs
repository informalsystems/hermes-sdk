use cgp_core::prelude::*;
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::impls::queries::query_and_convert_consensus_state::QueryAndConvertRawConsensusState;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerierComponent;

use crate::cosmos::impls::queries::consensus_state_heights::QuerySovereignConsensusStateHeightsFromGrpc;
use crate::cosmos::impls::sovereign_to_cosmos::client::create_client_message::BuildCreateSovereignClientMessageOnCosmos;
use crate::cosmos::impls::sovereign_to_cosmos::client::update_client_message::BuildUpdateSovereignClientMessageOnCosmos;
use crate::cosmos::impls::sovereign_to_cosmos::connection_handshake_message::BuildSovereignConnectionHandshakeMessageOnCosmos;

use super::impls::sovereign_to_cosmos::channel_handshake_message::BuildSovereignChannelHandshakeMessageOnCosmos;

pub struct SovereignCosmosComponents;

delegate_components! {
    SovereignCosmosComponents {
        ClientStateQuerierComponent:
            QueryAndConvertRawClientState,
        ConsensusStateQuerierComponent:
            QueryAndConvertRawConsensusState,
        UpdateClientMessageBuilderComponent:
            BuildUpdateSovereignClientMessageOnCosmos,
        CreateClientMessageBuilderComponent:
            BuildCreateSovereignClientMessageOnCosmos,
        ConnectionHandshakeMessageBuilderComponent:
            BuildSovereignConnectionHandshakeMessageOnCosmos,
        ChannelHandshakeMessageBuilderComponent:
            BuildSovereignChannelHandshakeMessageOnCosmos,
        ConsensusStateHeightsQuerierComponent:
            QuerySovereignConsensusStateHeightsFromGrpc,
    }
}
