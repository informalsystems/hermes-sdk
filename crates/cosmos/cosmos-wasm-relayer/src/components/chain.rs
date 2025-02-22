#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_cosmos_relayer::presets::chain::re_exports::*;
    use hermes_cosmos_relayer::presets::chain::{
        with_cosmos_chain_full_preset, CosmosChainFullPreset,
    };

    use crate::impls::client_state::ProvideWrappedTendermintClientState;

    with_cosmos_chain_full_preset! {
        [
            ClientStateTypeComponent,
            ClientStateFieldsComponent,
        ],
        | Components | {
            cgp_preset! {
                CosmosChainWasmPreset {
                    Components : CosmosChainFullPreset,
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
