#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_test_components::bootstrap::traits::chain::{
        CanBootstrapChain, ChainBootstrapperComponent,
    };

    use crate::bootstrap::impls::chain::bootstrap_chain::BootstrapCosmosChain;
    use crate::bootstrap::impls::chain::start_chain::StartCosmosChain;
    use crate::bootstrap::impls::fields::genesis_denom::GetCosmosGenesisDenoms;
    use crate::bootstrap::impls::fields::hd_path::ProvideCosmosHdPath;
    use crate::bootstrap::impls::generator::random_chain_id::GenerateRandomChainId;
    use crate::bootstrap::impls::genesis::add_genesis_account::AddCosmosGenesisAccount;
    use crate::bootstrap::impls::genesis::add_genesis_validator::AddCosmosGenesisValidator;
    use crate::bootstrap::impls::genesis::add_genesis_wallet::AddCosmosWalletToGenesis;
    use crate::bootstrap::impls::genesis::collect_gentxs::CollectCosmosGentxs;
    use crate::bootstrap::impls::initializers::create_chain_home_dir::CreateChainHomeDirFromTestDir;
    use crate::bootstrap::impls::initializers::init_chain_data::InitCosmosChainData;
    use crate::bootstrap::impls::initializers::init_wallet::{GetStdOut, InitCosmosTestWallet};
    use crate::bootstrap::impls::initializers::update_chain_config::UpdateCosmosChainNodeConfig;
    use crate::bootstrap::impls::initializers::update_genesis_config::UpdateCosmosGenesisConfig;
    use crate::bootstrap::impls::types::chain_node_config::ProvideCosmosChainNodeConfigType;
    use crate::bootstrap::impls::types::genesis_config::ProvideCosmosGenesisConfigType;
    use crate::bootstrap::impls::types::wallet_config::ProvideCosmosWalletConfigType;
    use crate::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
    use crate::bootstrap::traits::fields::denom::{
        DenomForStaking, DenomForTransfer, DenomPrefixGetter, GenesisDenomGetterComponent,
    };
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
    use crate::bootstrap::traits::types::chain_node_config::{
        ChainNodeConfigTypeComponent, ProvideChainNodeConfigType,
    };
    use crate::bootstrap::traits::types::genesis_config::{
        ChainGenesisConfigTypeComponent, ProvideChainGenesisConfigType,
    };
    use crate::bootstrap::traits::types::wallet_config::{
        ProvideWalletConfigType, WalletConfigFieldsComponent, WalletConfigFieldsGetter,
        WalletConfigTypeComponent,
    };

    cgp_preset! {
        CosmosSdkBootstrapComponents {
            GenesisAccountAdderComponent: AddCosmosGenesisAccount,
            GenesisValidatorAdderComponent: AddCosmosGenesisValidator,
            GenesisTransactionsCollectorComponent: CollectCosmosGentxs,
            WalletInitializerComponent: InitCosmosTestWallet<GetStdOut>,

            ChainNodeConfigTypeComponent: ProvideCosmosChainNodeConfigType,
            ChainGenesisConfigTypeComponent: ProvideCosmosGenesisConfigType,
            [
                WalletConfigTypeComponent,
                WalletConfigFieldsComponent,
            ]: ProvideCosmosWalletConfigType,
            ChainIdGeneratorComponent: GenerateRandomChainId,
            ChainHomeDirInitializerComponent: CreateChainHomeDirFromTestDir,
            ChainDataInitializerComponent: InitCosmosChainData,
            WalletHdPathComponent: ProvideCosmosHdPath,
            GenesisDenomGetterComponent: GetCosmosGenesisDenoms,
            ChainNodeConfigInitializerComponent: UpdateCosmosChainNodeConfig,
            ChainGenesisConfigInitializerComponent: UpdateCosmosGenesisConfig,
            GenesisWalletAdderComponent: AddCosmosWalletToGenesis,
            ChainFullNodeStarterComponent: StartCosmosChain,
            ChainBootstrapperComponent: BootstrapCosmosChain,
        }
    }
}
