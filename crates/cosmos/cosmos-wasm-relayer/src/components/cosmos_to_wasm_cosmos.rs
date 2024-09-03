use cgp::prelude::*;
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::CosmosToCosmosComponents;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilderComponent, ChannelOpenConfirmMessageBuilderComponent,
    ChannelOpenInitMessageBuilderComponent, ChannelOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
    ClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerierComponent;
use hermes_relayer_components::chain::traits::types::create_client::CreateClientMessageOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetterComponent;

use crate::impls::create_client_message::BuildCreateWasmTendermintClientMessage;
use crate::impls::update_client_message::BuildUpdateWasmTendermintClientMessage;
use crate::types::create_client::ProvidCreateWasmTendermintMessageOptionsType;

define_components! {
    CosmosToWasmCosmosComponents {
        CreateClientMessageBuilderComponent:
            BuildCreateWasmTendermintClientMessage,
        CreateClientMessageOptionsTypeComponent:
            ProvidCreateWasmTendermintMessageOptionsType,
        UpdateClientMessageBuilderComponent:
            BuildUpdateWasmTendermintClientMessage,
        [
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
            AllClientStatesQuerierComponent,
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
            ChannelOpenInitMessageBuilderComponent,
            ChannelOpenTryMessageBuilderComponent,
            ChannelOpenAckMessageBuilderComponent,
            ChannelOpenConfirmMessageBuilderComponent,
            ConsensusStateHeightsQuerierComponent,
            CounterpartyMessageHeightGetterComponent,
        ]:
            CosmosToCosmosComponents,
    }
}
