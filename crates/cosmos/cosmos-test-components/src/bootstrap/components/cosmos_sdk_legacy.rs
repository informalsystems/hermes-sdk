#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use CosmosSdkBootstrapComponents::re_exports::*;

    use crate::bootstrap::components::cosmos_sdk::CosmosSdkBootstrapComponents;
    use crate::bootstrap::impls::genesis_legacy::add_genesis_account::LegacyAddCosmosGenesisAccount;
    use crate::bootstrap::impls::genesis_legacy::add_genesis_validator::LegacyAddCosmosGenesisValidator;
    use crate::bootstrap::impls::genesis_legacy::collect_gentxs::LegacyCollectCosmosGentxs;
    use crate::bootstrap::impls::initializers::init_wallet::GetStdOutOrElseStdErr;

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
