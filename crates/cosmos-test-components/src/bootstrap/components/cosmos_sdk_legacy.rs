use core::marker::PhantomData;

use cgp_core::prelude::*;
use ibc_test_components::bootstrap::traits::chain::ChainBootstrapperComponent;
use ibc_test_components::bootstrap::traits::types::chain::ChainTypeComponent;

use crate::bootstrap::components::cosmos_sdk::CosmosSdkBootstrapComponents;
use crate::bootstrap::impls::genesis_legacy::add_genesis_account::LegacyAddCosmosGenesisAccount;
use crate::bootstrap::impls::genesis_legacy::add_genesis_validator::LegacyAddCosmosGenesisValidator;
use crate::bootstrap::impls::genesis_legacy::collect_gentxs::LegacyCollectCosmosGentxs;
use crate::bootstrap::traits::chain::build_chain::ChainFromBootstrapParamsBuilderComponent;
use crate::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
use crate::bootstrap::traits::fields::chain_command_path::ChainCommandPathComponent;
use crate::bootstrap::traits::fields::hd_path::WalletHdPathComponent;
use crate::bootstrap::traits::fields::random_id::RandomIdFlagComponent;
use crate::bootstrap::traits::fields::test_dir::TestDirComponent;
use crate::bootstrap::traits::generator::generate_chain_id::ChainIdGeneratorComponent;
use crate::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use crate::bootstrap::traits::genesis::add_genesis_account::GenesisAccountAdderComponent;
use crate::bootstrap::traits::genesis::add_genesis_validator::GenesisValidatorAdderComponent;
use crate::bootstrap::traits::genesis::add_genesis_wallet::GenesisWalletAdderComponent;
use crate::bootstrap::traits::genesis::collect_gentxs::GenesisTransactionsCollectorComponent;
use crate::bootstrap::traits::initializers::init_chain_config::ChainConfigInitializerComponent;
use crate::bootstrap::traits::initializers::init_chain_data::ChainDataInitializerComponent;
use crate::bootstrap::traits::initializers::init_chain_home_dir::ChainHomeDirInitializerComponent;
use crate::bootstrap::traits::initializers::init_genesis_config::GenesisConfigInitializerComponent;
use crate::bootstrap::traits::initializers::init_wallet::WalletInitializerComponent;
use crate::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifierComponent;
use crate::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifierComponent;
use crate::bootstrap::traits::types::chain_config::ChainConfigTypeComponent;
use crate::bootstrap::traits::types::genesis_config::GenesisConfigTypeComponent;
use crate::bootstrap::traits::types::wallet_config::{
    WalletConfigFieldsComponent, WalletConfigTypeComponent,
};

pub struct LegacyCosmosSdkBootstrapComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    LegacyCosmosSdkBootstrapComponents<BaseComponents>;

    // Components that differ from `CosmosSdkBootstrapComponents`
    GenesisAccountAdderComponent: LegacyAddCosmosGenesisAccount,
    GenesisValidatorAdderComponent: LegacyAddCosmosGenesisValidator,
    GenesisTransactionsCollectorComponent: LegacyCollectCosmosGentxs,

    // Components that are the same as `CosmosSdkBootstrapComponents`
    [
        ChainIdGeneratorComponent,
        ChainHomeDirInitializerComponent,
        ChainDataInitializerComponent,
        WalletHdPathComponent,
        WalletInitializerComponent,
        ChainConfigInitializerComponent,
        GenesisConfigInitializerComponent,
        GenesisWalletAdderComponent,
        ChainFullNodeStarterComponent,
        ChainBootstrapperComponent,

        // Components that are forwarded to `BaseComponents` via `CosmosSdkBootstrapComponents`
        ChainTypeComponent,
        GenesisConfigTypeComponent,
        ChainConfigTypeComponent,
        TestDirComponent,
        ChainCommandPathComponent,
        RandomIdFlagComponent,
        WalletConfigTypeComponent,
        WalletConfigFieldsComponent,
        WalletConfigGeneratorComponent,
        CosmosGenesisConfigModifierComponent,
        CometConfigModifierComponent,
        ChainFromBootstrapParamsBuilderComponent,
    ]:
        CosmosSdkBootstrapComponents<BaseComponents>,
);
