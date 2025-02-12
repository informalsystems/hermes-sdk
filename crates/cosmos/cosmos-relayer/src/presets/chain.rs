use cgp::prelude::*;
pub use hermes_cosmos_chain_components::components::client::*;
pub use hermes_cosmos_chain_components::components::transaction::*;
pub use hermes_cosmos_test_components::chain::components::*;

with_cosmos_chain_client_preset! {
    | ClientComponents | {
        with_cosmos_chain_tx_preset! {
            | TxComponents | {
                with_cosmmos_chain_test_preset! {
                    | TestComponents | {
                        cgp_preset! {
                            CosmosChainFullPreset {
                                ClientComponents: CosmosChainClientPreset,
                                TestComponents: CosmmosChainTestPreset,
                                TxComponents: CosmosChainTxPreset,
                            }
                        }
                    }
                }
            }
        }
    }
}
