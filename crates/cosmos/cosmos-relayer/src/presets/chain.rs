use cgp::prelude::*;
pub use hermes_cosmos_chain_components::components::client::*;
pub use hermes_cosmos_chain_components::components::transaction::*;
pub use hermes_cosmos_test_components::chain::components::*;

CosmosChainClientPreset::with_components! {
    | ClientComponents | {
        CosmosChainTxPreset::with_components! {
            | TxComponents | {
                CosmosChainTestPreset::with_components! {
                    | TestComponents | {
                        cgp_preset! {
                            CosmosChainFullPreset {
                                ClientComponents: CosmosChainClientPreset::Provider,
                                TestComponents: CosmosChainTestPreset::Provider,
                                TxComponents: CosmosChainTxPreset::Provider,
                            }
                        }
                    }
                }
            }
        }
    }
}
