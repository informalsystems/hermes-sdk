use cgp::prelude::*;
pub use hermes_cosmos_chain_components::components::client::*;
pub use hermes_cosmos_chain_components::components::transaction::*;
pub use hermes_cosmos_test_components::chain::components::*;

with_cosmos_client_components! {
    | ClientComponents | {
        with_cosmos_tx_components! {
            | TxComponents | {
                with_cosmmos_chain_test_components! {
                    | TestComponents | {
                        cgp_preset! {
                            CosmosChainFullPreset {
                                ClientComponents: CosmosClientComponents,
                                TestComponents: CosmmosChainTestComponents,
                                TxComponents: CosmosTxComponents,
                            }
                        }
                    }
                }
            }
        }
    }
}
