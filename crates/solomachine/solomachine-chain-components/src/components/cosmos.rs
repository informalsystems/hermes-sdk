use hermes_cosmos_chain_preset::presets::CosmosToCosmosComponents;
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::{
    ClientStateFieldsComponent, ClientStateQuerierComponent, ClientStateTypeComponent,
    ClientStateWithProofsQuerierComponent, ConnectionOpenAckMessageBuilderComponent,
    ConnectionOpenConfirmMessageBuilderComponent, ConnectionOpenInitMessageBuilderComponent,
    ConnectionOpenTryMessageBuilderComponent, ConsensusStateQuerierComponent,
    ConsensusStateWithProofsQuerierComponent, CreateClientMessageBuilderComponent,
    CreateClientMessageOptionsTypeComponent, CreateClientPayloadBuilderComponent,
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
    UpdateClientPayloadBuilderComponent, UpdateClientPayloadTypeComponent,
};

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
