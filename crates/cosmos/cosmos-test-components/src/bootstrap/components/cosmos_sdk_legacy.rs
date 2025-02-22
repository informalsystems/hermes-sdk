#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_test_components::bootstrap::traits::chain::ChainBootstrapperComponent;

    use crate::bootstrap::components::cosmos_sdk::{CosmosSdkBootstrapComponents, *};
    use crate::bootstrap::impls::genesis_legacy::add_genesis_account::LegacyAddCosmosGenesisAccount;
    use crate::bootstrap::impls::genesis_legacy::add_genesis_validator::LegacyAddCosmosGenesisValidator;
    use crate::bootstrap::impls::genesis_legacy::collect_gentxs::LegacyCollectCosmosGentxs;
    use crate::bootstrap::impls::initializers::init_wallet::{
        GetStdOutOrElseStdErr, InitCosmosTestWallet,
    };
    use crate::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
    use crate::bootstrap::traits::fields::denom::GenesisDenomGetterComponent;
    use crate::bootstrap::traits::fields::hd_path::WalletHdPathComponent;
    use crate::bootstrap::traits::generator::generate_chain_id::ChainIdGeneratorComponent;
    use crate::bootstrap::traits::genesis::add_genesis_account::GenesisAccountAdderComponent;
    use crate::bootstrap::traits::genesis::add_genesis_validator::GenesisValidatorAdderComponent;
    use crate::bootstrap::traits::genesis::add_genesis_wallet::GenesisWalletAdderComponent;
    use crate::bootstrap::traits::genesis::collect_gentxs::GenesisTransactionsCollectorComponent;
    use crate::bootstrap::traits::initializers::init_chain_config::ChainNodeConfigInitializerComponent;
    use crate::bootstrap::traits::initializers::init_chain_data::ChainDataInitializerComponent;
    use crate::bootstrap::traits::initializers::init_chain_home_dir::ChainHomeDirInitializerComponent;
    use crate::bootstrap::traits::initializers::init_genesis_config::ChainGenesisConfigInitializerComponent;
    use crate::bootstrap::traits::initializers::init_wallet::WalletInitializerComponent;
    use crate::bootstrap::traits::types::chain_node_config::ChainNodeConfigTypeComponent;
    use crate::bootstrap::traits::types::genesis_config::ChainGenesisConfigTypeComponent;
    use crate::bootstrap::traits::types::wallet_config::{
        WalletConfigFieldsComponent, WalletConfigTypeComponent,
    };

    with_cosmos_sdk_bootstrap_components! {
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

                    Components: CosmosSdkBootstrapComponents,
                }
            }
        }
    }
}
