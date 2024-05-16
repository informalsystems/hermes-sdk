use cgp_core::prelude::*;
use hermes_cosmos_chain_components::impls::connection::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::impls::queries::query_and_convert_consensus_state::QueryAndConvertRawConsensusState;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerierComponent;

use crate::cosmos::impls::queries::consensus_state_heights::QuerySovereignConsensusStateHeightsFromGrpc;
use crate::cosmos::impls::sovereign_to_cosmos::client::create_client_message::BuildCreateSovereignClientMessageOnCosmos;
use crate::cosmos::impls::sovereign_to_cosmos::client::update_client_message::BuildUpdateSovereignClientMessageOnCosmos;
use crate::cosmos::impls::sovereign_to_cosmos::connection_handshake_message::BuildSovereignConnectionHandshakeMessageOnCosmos;

use super::impls::sovereign_to_cosmos::channel_handshake_message::BuildSovereignChannelHandshakeMessageOnCosmos;

pub struct SovereignCosmosComponents;

delegate_components! {
    SovereignCosmosComponents {
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
        UpdateClientMessageBuilderComponent:
            BuildUpdateSovereignClientMessageOnCosmos,
        CreateClientMessageBuilderComponent:
            BuildCreateSovereignClientMessageOnCosmos,
        [
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
        ]:
            BuildSovereignConnectionHandshakeMessageOnCosmos,
        [
            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
        ]:
            BuildCosmosConnectionHandshakeMessage,
        ChannelHandshakeMessageBuilderComponent:
            BuildSovereignChannelHandshakeMessageOnCosmos,
        ConsensusStateHeightsQuerierComponent:
            QuerySovereignConsensusStateHeightsFromGrpc,
    }
}
