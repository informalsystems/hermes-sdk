#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_cosmos_chain_preset::presets::CosmosChainPreset;
    use CosmosChainPreset::re_exports::*;

    use crate::impls::client_state::ProvideWrappedTendermintClientState;

    CosmosChainPreset::with_components! {
        [
            ClientStateTypeComponent,
            ClientStateFieldsComponent,
        ],
        | Components | {
            cgp_preset! {
                CosmosChainWasmPreset {
                    Components : CosmosChainFullPreset::Provider,
                    [
                        ClientStateTypeComponent,
                        ClientStateFieldsComponent,
                    ]:
                        ProvideWrappedTendermintClientState,
                }
            }
        }
    }
}
