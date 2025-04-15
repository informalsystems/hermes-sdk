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

    use crate::impls::{
        BuildCreateWasmTendermintClientMessage, BuildUpdateWasmTendermintClientMessage,
    };
    use crate::types::ProvidCreateWasmTendermintMessageOptionsType;

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
