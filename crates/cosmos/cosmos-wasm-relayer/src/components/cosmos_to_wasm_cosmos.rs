#[cgp::re_export_imports]
mod preset {
    use hermes_core::relayer_components::chain::traits::{
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
    use hermes_cosmos_core::chain_preset::presets::CosmosToCosmosComponents;
    use hermes_prelude::*;
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
            ClientRecoveryComponent,
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
                    ClientRecoveryComponent:
                        RecoverClientWithGovernanceProposal,
                    Components:
                        CosmosToCosmosComponents::Provider,
                }
            }
        }
    }
}
