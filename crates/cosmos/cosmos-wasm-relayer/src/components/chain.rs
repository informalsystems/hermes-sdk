#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_cosmos_chain_preset::presets::CosmosChainPreset;
    use CosmosChainPreset::re_exports::*;

    use crate::impls::ProvideWrappedTendermintClientState;

    CosmosChainPreset::with_components! {
        [
            ClientStateTypeComponent,
            ClientStateFieldsComponent,
        ],
        | Components | {
            cgp_preset! {
                CosmosChainWasmPreset {
                    Components : CosmosChainPreset::Provider,
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
