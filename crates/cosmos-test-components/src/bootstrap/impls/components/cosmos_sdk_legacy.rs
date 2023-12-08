use core::marker::PhantomData;

use cgp_core::prelude::*;
use ibc_test_components::bootstrap::traits::chain::ChainBootstrapperComponent;

use crate::bootstrap::impls::components::cosmos_sdk::CosmosSdkBootstrapComponents;
use crate::bootstrap::impls::genesis_legacy::add_genesis_account::LegacyAddCosmosGenesisAccount;
use crate::bootstrap::impls::genesis_legacy::add_genesis_validator::LegacyAddCosmosGenesisValidator;
use crate::bootstrap::impls::genesis_legacy::collect_gentxs::LegacyCollectCosmosGentxs;
use crate::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
use crate::bootstrap::traits::genesis::add_genesis_account::GenesisAccountAdderComponent;
use crate::bootstrap::traits::genesis::add_genesis_validator::GenesisValidatorAdderComponent;
use crate::bootstrap::traits::genesis::add_genesis_wallet::GenesisWalletAdderComponent;
use crate::bootstrap::traits::genesis::collect_gentxs::GenesisTransactionsCollectorComponent;
use crate::bootstrap::traits::initializers::init_chain_config::ChainConfigInitializerComponent;
use crate::bootstrap::traits::initializers::init_chain_data::ChainDataInitializerComponent;
use crate::bootstrap::traits::initializers::init_genesis_config::GenesisConfigInitializerComponent;
use crate::bootstrap::traits::initializers::init_wallet::WalletInitializerComponent;

pub struct LegacyCosmosSdkBootstrapComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    LegacyCosmosSdkBootstrapComponents<BaseComponents>;
    GenesisAccountAdderComponent: LegacyAddCosmosGenesisAccount,
    GenesisValidatorAdderComponent: LegacyAddCosmosGenesisValidator,
    GenesisTransactionsCollectorComponent: LegacyCollectCosmosGentxs,
    [
        ChainDataInitializerComponent,
        WalletInitializerComponent,
        ChainConfigInitializerComponent,
        GenesisConfigInitializerComponent,
        GenesisWalletAdderComponent,
        ChainFullNodeStarterComponent,
        ChainBootstrapperComponent,
    ]: CosmosSdkBootstrapComponents<BaseComponents>,
);
