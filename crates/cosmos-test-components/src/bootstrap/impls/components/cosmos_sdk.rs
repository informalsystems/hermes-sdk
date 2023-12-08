use core::marker::PhantomData;

use cgp_core::prelude::*;
use ibc_test_components::bootstrap::traits::chain::ChainBootstrapperComponent;

use crate::bootstrap::impls::chain::bootstrap_chain::BootstrapCosmosChain;
use crate::bootstrap::impls::chain::start_chain::StartCosmosChain;
use crate::bootstrap::impls::genesis::add_genesis_account::AddCosmosGenesisAccount;
use crate::bootstrap::impls::genesis::add_genesis_validator::AddCosmosGenesisValidator;
use crate::bootstrap::impls::genesis::add_genesis_wallet::AddCosmosWalletToGenesis;
use crate::bootstrap::impls::genesis::collect_gentxs::CollectCosmosGentxs;
use crate::bootstrap::impls::initializers::init_chain_data::InitCosmosChainData;
use crate::bootstrap::impls::initializers::init_wallet::InitCosmosTestWallet;
use crate::bootstrap::impls::initializers::update_chain_config::UpdateCosmosChainConfig;
use crate::bootstrap::impls::initializers::update_genesis_config::UpdateCosmosGenesisConfig;
use crate::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
use crate::bootstrap::traits::genesis::add_genesis_account::GenesisAccountAdderComponent;
use crate::bootstrap::traits::genesis::add_genesis_validator::GenesisValidatorAdderComponent;
use crate::bootstrap::traits::genesis::add_genesis_wallet::GenesisWalletAdderComponent;
use crate::bootstrap::traits::genesis::collect_gentxs::GenesisTransactionsCollectorComponent;
use crate::bootstrap::traits::initializers::init_chain_config::ChainConfigInitializerComponent;
use crate::bootstrap::traits::initializers::init_chain_data::ChainDataInitializerComponent;
use crate::bootstrap::traits::initializers::init_genesis_config::GenesisConfigInitializerComponent;
use crate::bootstrap::traits::initializers::init_wallet::WalletInitializerComponent;

pub struct CosmosSdkBootstrapComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    CosmosSdkBootstrapComponents<BaseComponents>;

    // Components that will be swapped in `LegacyCosmosSdkBootstrapComponents`
    GenesisAccountAdderComponent: AddCosmosGenesisAccount,
    GenesisValidatorAdderComponent: AddCosmosGenesisValidator,
    GenesisTransactionsCollectorComponent: CollectCosmosGentxs,

    // Components that are common with `LegacyCosmosSdkBootstrapComponents`
    ChainDataInitializerComponent: InitCosmosChainData,
    WalletInitializerComponent: InitCosmosTestWallet,
    ChainConfigInitializerComponent: UpdateCosmosChainConfig,
    GenesisConfigInitializerComponent: UpdateCosmosGenesisConfig,
    GenesisWalletAdderComponent: AddCosmosWalletToGenesis,
    ChainFullNodeStarterComponent: StartCosmosChain,
    ChainBootstrapperComponent: BootstrapCosmosChain,
);
