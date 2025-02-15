use cgp::prelude::*;
use hermes_cosmos_relayer::presets::chain::*;

use crate::impls::client_state::ProvideWrappedTendermintClientState;

CosmosChainFullPreset::with_components! {
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
