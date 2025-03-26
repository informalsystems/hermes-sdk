#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_cosmos_chain_preset::presets::CosmosToCosmosComponents;
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
    use CosmosToCosmosComponents::re_exports::*;

    use crate::impls::create_client_message::BuildCreateWasmTendermintClientMessage;
    use crate::impls::update_client_message::BuildUpdateWasmTendermintClientMessage;
    use crate::types::create_client::ProvidCreateWasmTendermintMessageOptionsType;

    CosmosToCosmosComponents::with_components! {
        [
            CreateClientMessageBuilderComponent,
            CreateClientMessageOptionsTypeComponent,
            UpdateClientMessageBuilderComponent,
        ],
        | Components | {
            cgp_preset! {
                CosmosToWasmCosmosComponents {
                    CreateClientMessageBuilderComponent:
                        BuildCreateWasmTendermintClientMessage,
                    CreateClientMessageOptionsTypeComponent:
                        ProvidCreateWasmTendermintMessageOptionsType,
                    UpdateClientMessageBuilderComponent:
                        BuildUpdateWasmTendermintClientMessage,
                    Components:
                        CosmosToCosmosComponents::Provider,
                }
            }
        }
    }
}
