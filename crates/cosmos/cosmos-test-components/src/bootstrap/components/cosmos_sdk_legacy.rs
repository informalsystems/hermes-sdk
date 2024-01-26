use core::fmt::Display;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::bootstrap::traits::chain::{
    CanBootstrapChain, ChainBootstrapperComponent,
};
use hermes_test_components::chain_driver::traits::build::chain_id::CanBuildChainIdFromString;
use hermes_test_components::chain_driver::traits::types::address::HasAddressType;
use hermes_test_components::chain_driver::traits::types::amount::HasAmountType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::chain_driver::traits::types::chain::ProvideChainType;
use hermes_test_components::chain_driver::traits::types::wallet::HasWalletType;
use hermes_test_components::driver::traits::types::chain_driver::ProvideChainDriverType;
use hermes_test_components::runtime::traits::child_process::CanStartChildProcess;
use hermes_test_components::runtime::traits::create_dir::CanCreateDir;
use hermes_test_components::runtime::traits::exec_command::CanExecCommand;
use hermes_test_components::runtime::traits::read_file::CanReadFileAsString;
use hermes_test_components::runtime::traits::reserve_port::CanReserveTcpPort;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;
use ibc_relayer::keyring::errors::Error as KeyringError;

use crate::bootstrap::components::cosmos_sdk::CosmosSdkBootstrapComponents;
use crate::bootstrap::impls::genesis_legacy::add_genesis_account::LegacyAddCosmosGenesisAccount;
use crate::bootstrap::impls::genesis_legacy::add_genesis_validator::LegacyAddCosmosGenesisValidator;
use crate::bootstrap::impls::genesis_legacy::collect_gentxs::LegacyCollectCosmosGentxs;
use crate::bootstrap::traits::chain::build_chain::ChainFromBootstrapParamsBuilder;
use crate::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
use crate::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use crate::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetter;
use crate::bootstrap::traits::fields::hd_path::WalletHdPathComponent;
use crate::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
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
use crate::bootstrap::traits::types::wallet_config::{
    ProvideWalletConfigType, WalletConfigFieldsGetter,
};
use crate::bootstrap::types::chain_config::CosmosChainConfig;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;
use crate::chain_driver::types::wallet::CosmosTestWallet;

pub struct LegacyCosmosSdkBootstrapComponents;

delegate_components! {
    #[mark_component(IsLegacyCosmosSdkBootstrapComponent)]
    #[mark_delegate(DelegatesToLegacyToCosmosSdkBootstrapComponents)]
    LegacyCosmosSdkBootstrapComponents {
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
        ]:
            CosmosSdkBootstrapComponents,
    }
}

pub trait CanUseLegacyCosmosSdkChainBootstrapper: UseLegacyCosmosSdkChainBootstrapper {}

pub trait UseLegacyCosmosSdkChainBootstrapper: CanBootstrapChain {}

impl<Bootstrap, Runtime, Chain, ChainDriver, Components> UseLegacyCosmosSdkChainBootstrapper
    for Bootstrap
where
    Bootstrap: HasComponents<Components = Components>
        + HasRuntime<Runtime = Runtime>
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<&'static str>
        + CanRaiseError<KeyringError>
        + CanRaiseError<serde_json::Error>
        + CanRaiseError<toml::ser::Error>
        + CanRaiseError<toml::de::Error>,
    Components: DelegatesToLegacyToCosmosSdkBootstrapComponents
        + ProvideChainType<Bootstrap, Chain = Chain>
        + ProvideChainDriverType<Bootstrap, ChainDriver = ChainDriver>
        + ProvideGenesisConfigType<Bootstrap, GenesisConfig = CosmosGenesisConfig>
        + ProvideChainConfigType<Bootstrap, ChainConfig = CosmosChainConfig>
        + ChainStoreDirGetter<Bootstrap>
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
        + CanCreateDir
        + CanReserveTcpPort,
    Chain: HasChainIdType,
    ChainDriver: HasChainType<Chain = Chain>
        + HasWalletType<Wallet = CosmosTestWallet>
        + HasAmountType
        + HasAddressType
        + CanBuildChainIdFromString,
    Chain::ChainId: Display,
{
}
