use cgp::prelude::*;
pub use hermes_test_components::bootstrap::traits::chain::{
    CanBootstrapChain, ChainBootstrapperComponent,
};

use crate::bootstrap::components::cosmos_sdk::{CosmosSdkBootstrapComponents, *};
use crate::bootstrap::impls::genesis_legacy::add_genesis_account::LegacyAddCosmosGenesisAccount;
use crate::bootstrap::impls::genesis_legacy::add_genesis_validator::LegacyAddCosmosGenesisValidator;
use crate::bootstrap::impls::genesis_legacy::collect_gentxs::LegacyCollectCosmosGentxs;
use crate::bootstrap::impls::initializers::init_wallet::{
    GetStdOutOrElseStdErr, InitCosmosTestWallet,
};
pub use crate::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
pub use crate::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, DenomPrefixGetter, GenesisDenomGetterComponent,
};
pub use crate::bootstrap::traits::fields::hd_path::WalletHdPathComponent;
pub use crate::bootstrap::traits::generator::generate_chain_id::ChainIdGeneratorComponent;
pub use crate::bootstrap::traits::generator::generate_wallet_config::WalletConfigGenerator;
pub use crate::bootstrap::traits::genesis::add_genesis_account::GenesisAccountAdderComponent;
pub use crate::bootstrap::traits::genesis::add_genesis_validator::GenesisValidatorAdderComponent;
pub use crate::bootstrap::traits::genesis::add_genesis_wallet::GenesisWalletAdderComponent;
pub use crate::bootstrap::traits::genesis::collect_gentxs::GenesisTransactionsCollectorComponent;
pub use crate::bootstrap::traits::initializers::init_chain_config::ChainNodeConfigInitializerComponent;
pub use crate::bootstrap::traits::initializers::init_chain_data::ChainDataInitializerComponent;
pub use crate::bootstrap::traits::initializers::init_chain_home_dir::ChainHomeDirInitializerComponent;
pub use crate::bootstrap::traits::initializers::init_genesis_config::ChainGenesisConfigInitializerComponent;
pub use crate::bootstrap::traits::initializers::init_wallet::WalletInitializerComponent;
pub use crate::bootstrap::traits::types::chain_node_config::{
    ChainNodeConfigTypeComponent, ProvideChainNodeConfigType,
};
pub use crate::bootstrap::traits::types::genesis_config::{
    ChainGenesisConfigTypeComponent, ProvideChainGenesisConfigType,
};
pub use crate::bootstrap::traits::types::wallet_config::{
    ProvideWalletConfigType, WalletConfigFieldsComponent, WalletConfigFieldsGetter,
    WalletConfigTypeComponent,
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
