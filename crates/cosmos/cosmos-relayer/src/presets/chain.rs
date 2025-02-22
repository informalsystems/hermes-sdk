#[cgp::re_export_imports]
mod preset {
    use hermes_cosmos_chain_components::components::client::re_exports::*;
    use hermes_cosmos_chain_components::components::client::{
        with_cosmos_chain_client_preset, CosmosChainClientPreset,
    };
    use hermes_cosmos_chain_components::components::transaction::re_exports::*;
    use hermes_cosmos_chain_components::components::transaction::{
        with_cosmos_chain_tx_preset, CosmosChainTxPreset,
    };
    use hermes_cosmos_test_components::chain::components::re_exports::*;
    use hermes_cosmos_test_components::chain::components::{
        with_cosmmos_chain_test_preset, CosmmosChainTestPreset,
    };

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
}
