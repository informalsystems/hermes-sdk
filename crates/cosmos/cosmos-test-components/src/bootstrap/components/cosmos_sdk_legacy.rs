#[cgp::re_export_imports]
mod preset {
    use hermes_prelude::*;
    use CosmosSdkBootstrapComponents::re_exports::*;

    use crate::bootstrap::components::CosmosSdkBootstrapComponents;
    use crate::bootstrap::impls::{
        GetStdOutOrElseStdErr, LegacyAddCosmosGenesisAccount, LegacyAddCosmosGenesisValidator,
        LegacyCollectCosmosGentxs,
    };

    CosmosSdkBootstrapComponents::with_components! {
        [
            GenesisAccountAdderComponent,
            GenesisValidatorAdderComponent,
            GenesisTransactionsCollectorComponent,
            WalletInitializerComponent,
        ],
        | Components | {
            cgp_preset! {
                LegacyCosmosSdkBootstrapComponents {
                    GenesisAccountAdderComponent: LegacyAddCosmosGenesisAccount,
                    GenesisValidatorAdderComponent: LegacyAddCosmosGenesisValidator,
                    GenesisTransactionsCollectorComponent: LegacyCollectCosmosGentxs,
                    WalletInitializerComponent: InitCosmosTestWallet<GetStdOutOrElseStdErr>,

                    Components: CosmosSdkBootstrapComponents::Provider,
                }
            }
        }
    }
}
