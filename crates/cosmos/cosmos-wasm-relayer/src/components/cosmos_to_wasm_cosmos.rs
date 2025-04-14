#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_cosmos_chain_preset::presets::CosmosToCosmosComponents;
    use hermes_relayer_components::chain::traits::{
        AllClientStatesQuerierComponent, ChannelOpenAckMessageBuilderComponent,
        ChannelOpenConfirmMessageBuilderComponent, ChannelOpenInitMessageBuilderComponent,
        ChannelOpenTryMessageBuilderComponent, ClientStateQuerierComponent,
        ClientStateWithProofsQuerierComponent, ConnectionOpenAckMessageBuilderComponent,
        ConnectionOpenConfirmMessageBuilderComponent, ConnectionOpenInitMessageBuilderComponent,
        ConnectionOpenTryMessageBuilderComponent, ConsensusStateHeightsQuerierComponent,
        ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
        CounterpartyMessageHeightGetterComponent, CreateClientMessageBuilderComponent,
        CreateClientMessageOptionsTypeComponent, UpdateClientMessageBuilderComponent,
    };
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
