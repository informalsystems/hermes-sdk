use core::fmt::Display;
use std::path::Path;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use eyre::Report;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc_test_components::bootstrap::traits::chain::ChainBootstrapperComponent;
use ibc_test_components::bootstrap::traits::types::chain::ProvideChainType;
use ibc_test_components::chain::traits::build::CanBuildChainIdFromString;
use ibc_test_components::chain::traits::types::address::HasAddressType;
use ibc_test_components::chain::traits::types::amount::HasAmountType;
use ibc_test_components::chain::traits::types::wallet::HasWalletType;
use ibc_test_components::runtime::traits::child_process::CanStartChildProcess;
use ibc_test_components::runtime::traits::exec_command::CanExecCommand;
use ibc_test_components::runtime::traits::read_file::CanReadFileAsString;
use ibc_test_components::runtime::traits::reserve_port::CanReserveTcpPort;
use ibc_test_components::runtime::traits::write_file::CanWriteStringToFile;
use std::io::Error as IoError;

use crate::bootstrap::impls::chain::bootstrap_chain::BootstrapCosmosChain;
use crate::bootstrap::impls::chain::start_chain::StartCosmosChain;
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
use crate::bootstrap::traits::chain::build_chain::ChainFromBootstrapParamsBuilder;
use crate::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
use crate::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use crate::bootstrap::traits::fields::hd_path::WalletHdPathComponent;
use crate::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use crate::bootstrap::traits::fields::test_dir::TestDirGetter;
use crate::bootstrap::traits::generator::generate_chain_id::ChainIdGeneratorComponent;
use crate::bootstrap::traits::generator::generate_wallet_config::WalletConfigGenerator;
use crate::bootstrap::traits::genesis::add_genesis_account::GenesisAccountAdderComponent;
use crate::bootstrap::traits::genesis::add_genesis_validator::GenesisValidatorAdderComponent;
use crate::bootstrap::traits::genesis::add_genesis_wallet::GenesisWalletAdderComponent;
use crate::bootstrap::traits::genesis::collect_gentxs::GenesisTransactionsCollectorComponent;
use crate::bootstrap::traits::initializers::init_chain_config::ChainConfigInitializerComponent;
use crate::bootstrap::traits::initializers::init_chain_data::ChainDataInitializerComponent;
use crate::bootstrap::traits::initializers::init_chain_home_dir::ChainHomeDirInitializerComponent;
use crate::bootstrap::traits::initializers::init_genesis_config::GenesisConfigInitializerComponent;
use crate::bootstrap::traits::initializers::init_wallet::WalletInitializerComponent;
use crate::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;
use crate::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use crate::bootstrap::traits::types::chain_config::ProvideChainConfigType;
use crate::bootstrap::traits::types::genesis_config::ProvideGenesisConfigType;
use crate::bootstrap::traits::types::wallet_config::ProvideWalletConfigType;
use crate::bootstrap::traits::types::wallet_config::WalletConfigFieldsGetter;
use crate::bootstrap::types::chain_config::CosmosChainConfig;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;
use crate::chain::types::wallet::CosmosTestWallet;

pub struct CosmosSdkBootstrapComponents;

pub trait IsCosmosSdkBootstrapComponent<Component> {}

// Components that will be swapped in `LegacyCosmosSdkBootstrapComponents`
delegate_components!(
    CosmosSdkBootstrapComponents
        @markers[ IsCosmosSdkBootstrapComponent ]
    ;

    GenesisAccountAdderComponent: AddCosmosGenesisAccount,
    GenesisValidatorAdderComponent: AddCosmosGenesisValidator,
    GenesisTransactionsCollectorComponent: CollectCosmosGentxs,

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
);

pub trait CanUseCosmosSdkChainBootstrapper: UseCosmosSdkChainBootstrapper {}

pub trait UseCosmosSdkChainBootstrapper: CanBootstrapChain {}

impl<Bootstrap, Runtime, Chain, Components> UseCosmosSdkChainBootstrapper for Bootstrap
where
    Bootstrap: HasComponents<Components = Components>
        + HasRuntime<Runtime = Runtime>
        + HasErrorType
        + CanRaiseError<IoError>,
    Components: DelegateComponent<ChainIdGeneratorComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<ChainHomeDirInitializerComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<ChainDataInitializerComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<WalletHdPathComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<WalletInitializerComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<ChainConfigInitializerComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<
            GenesisConfigInitializerComponent,
            Delegate = CosmosSdkBootstrapComponents,
        > + DelegateComponent<GenesisWalletAdderComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<ChainFullNodeStarterComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<ChainBootstrapperComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<GenesisAccountAdderComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<GenesisValidatorAdderComponent, Delegate = CosmosSdkBootstrapComponents>
        + DelegateComponent<
            GenesisTransactionsCollectorComponent,
            Delegate = CosmosSdkBootstrapComponents,
        > + ProvideChainType<Bootstrap, Chain = Chain>
        + ProvideGenesisConfigType<Bootstrap>
        + ProvideChainConfigType<Bootstrap>
        + TestDirGetter<Bootstrap>
        + ChainCommandPathGetter<Bootstrap>
        + RandomIdFlagGetter<Bootstrap>
        + CosmosGenesisConfigModifier<Bootstrap>
        + CometConfigModifier<Bootstrap>
        + WalletConfigGenerator<Bootstrap>
        + ChainFromBootstrapParamsBuilder<Bootstrap>
        + ProvideWalletConfigType<Bootstrap>
        + WalletConfigFieldsGetter<Bootstrap>,
    Runtime: CanExecCommand
        + CanStartChildProcess
        + CanReadFileAsString
        + CanWriteStringToFile
        + CanReserveTcpPort,
    Chain: HasChainIdType
        + HasWalletType<Wallet = CosmosTestWallet>
        + HasAmountType
        + HasAddressType
        + CanBuildChainIdFromString,
    Chain::ChainId: Display,
    Runtime::FilePath: AsRef<Path>,
    Bootstrap::Error: From<Report>,
    Components::GenesisConfig: From<CosmosGenesisConfig>,
    Components::ChainConfig: From<CosmosChainConfig>,
{
}
