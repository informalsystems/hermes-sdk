#[cgp::re_export_imports]
mod preset {
    use hermes_cosmos_chain_components::components::client::CosmosChainClientPreset;
    use hermes_cosmos_chain_components::components::transaction::CosmosChainTxPreset;
    use hermes_cosmos_test_components::chain::components::CosmosChainTestPreset;
    use CosmosChainClientPreset::re_exports::*;
    use CosmosChainTestPreset::re_exports::*;
    use CosmosChainTxPreset::re_exports::*;

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
}
