use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use ibc_relayer_components::runtime::traits::runtime::RuntimeComponent;
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use ibc_test_components::bootstrap::traits::chain::ChainBootstrapperComponent;
use ibc_test_components::bootstrap::traits::types::chain::ChainTypeComponent;

use crate::bootstrap::impls::chain::bootstrap_chain::BootstrapCosmosChain;
use crate::bootstrap::impls::chain::start_chain::StartCosmosChain;
use crate::bootstrap::impls::fields::denom::GenesisDenomComponent;
use crate::bootstrap::impls::fields::hd_path::ProvideCosmosHdPath;
use crate::bootstrap::impls::generator::random_chain_id::GenerateRandomChainId;
use crate::bootstrap::impls::genesis::add_genesis_account::AddCosmosGenesisAccount;
use crate::bootstrap::impls::genesis::add_genesis_validator::AddCosmosGenesisValidator;
use crate::bootstrap::impls::genesis::add_genesis_wallet::AddCosmosWalletToGenesis;
use crate::bootstrap::impls::genesis::collect_gentxs::CollectCosmosGentxs;
use crate::bootstrap::impls::initializers::create_chain_home_dir::CreateChainHomeDirFromTestDir;
use crate::bootstrap::impls::initializers::init_chain_data::InitCosmosChainData;
use crate::bootstrap::impls::initializers::init_wallet::InitCosmosTestWallet;
use crate::bootstrap::impls::initializers::update_chain_config::UpdateCosmosChainConfig;
use crate::bootstrap::impls::initializers::update_genesis_config::UpdateCosmosGenesisConfig;
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

pub struct CosmosSdkBootstrapComponents<BaseComponents>(pub PhantomData<BaseComponents>);

pub trait IsCosmosSdkBootstrapComponent<Component> {}

pub trait IsCommonCosmosSdkBootstrapComponent<Component> {}

// Components that will be swapped in `LegacyCosmosSdkBootstrapComponents`
delegate_components!(
    CosmosSdkBootstrapComponents<BaseComponents>
        @markers[ IsCosmosSdkBootstrapComponent ]
    ;
    GenesisAccountAdderComponent: AddCosmosGenesisAccount,
    GenesisValidatorAdderComponent: AddCosmosGenesisValidator,
    GenesisTransactionsCollectorComponent: CollectCosmosGentxs,
);

// Components that are common with `LegacyCosmosSdkBootstrapComponents`
delegate_components!(
    CosmosSdkBootstrapComponents<BaseComponents>
        @markers[ IsCosmosSdkBootstrapComponent, IsCommonCosmosSdkBootstrapComponent ]
    ;
    ChainIdGeneratorComponent: GenerateRandomChainId,
    ChainHomeDirInitializerComponent: CreateChainHomeDirFromTestDir,
    ChainDataInitializerComponent: InitCosmosChainData,
    WalletHdPathComponent: ProvideCosmosHdPath,
    WalletInitializerComponent: InitCosmosTestWallet,
    ChainConfigInitializerComponent: UpdateCosmosChainConfig,
    GenesisConfigInitializerComponent: UpdateCosmosGenesisConfig,
    GenesisWalletAdderComponent: AddCosmosWalletToGenesis,
    ChainFullNodeStarterComponent: StartCosmosChain,
    ChainBootstrapperComponent: BootstrapCosmosChain,

    // Components that should be implemented by `BaseComponents`
    [
        ErrorTypeComponent,
        ErrorRaiserComponent,
        RuntimeTypeComponent,
        RuntimeComponent,
        ChainTypeComponent,
        GenesisConfigTypeComponent,
        ChainConfigTypeComponent,
        TestDirComponent,
        ChainCommandPathComponent,
        RandomIdFlagComponent,
        GenesisDenomComponent,
        WalletConfigTypeComponent,
        WalletConfigFieldsComponent,
        WalletConfigGeneratorComponent,
        CosmosGenesisConfigModifierComponent,
        CometConfigModifierComponent,
        ChainFromBootstrapParamsBuilderComponent,
    ]:
        BaseComponents,
);
