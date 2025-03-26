use cgp::prelude::*;
use hermes_cosmos_chain_preset::presets::CosmosToCosmosComponents;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsComponent, ClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsTypeComponent, CreateClientPayloadOptionsTypeComponent,
    CreateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;

use crate::impls::cosmos::connection_handshake_message::BuildSolomachineConnectionHandshakeMessagesForCosmos;
use crate::impls::cosmos::create_client_message::BuildCreateSolomachineClientMessage;

pub struct SolomachineCosmosComponents;

delegate_components! {
    SolomachineCosmosComponents {
        [
            ClientStateTypeComponent,
            ClientStateFieldsComponent,
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
            CreateClientPayloadOptionsTypeComponent,
            CreateClientPayloadBuilderComponent,
            UpdateClientPayloadBuilderComponent,
        ]:
            CosmosToCosmosComponents::Provider,
        [
            CreateClientMessageBuilderComponent,
            CreateClientMessageOptionsTypeComponent,
        ]:
            BuildCreateSolomachineClientMessage,
        [
            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
        ]:
            BuildSolomachineConnectionHandshakeMessagesForCosmos,
    }
}
