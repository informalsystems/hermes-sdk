use cgp_core::{CanRaiseError, HasComponents, HasErrorType};
use core::fmt::Display;
use eyre::Report;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::chain::CanBootstrapChain;
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
use std::path::Path;

use crate::bootstrap::components::cosmos_sdk_legacy::LegacyCosmosSdkBootstrapComponents;
use crate::bootstrap::traits::chain::build_chain::ChainFromBootstrapParamsBuilder;
use crate::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use crate::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use crate::bootstrap::traits::fields::test_dir::TestDirGetter;
use crate::bootstrap::traits::generator::generate_wallet_config::WalletConfigGenerator;
use crate::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;
use crate::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use crate::bootstrap::traits::types::chain_config::ProvideChainConfigType;
use crate::bootstrap::traits::types::genesis_config::ProvideGenesisConfigType;
use crate::bootstrap::traits::types::wallet_config::{
    ProvideWalletConfigType, WalletConfigFieldsGetter,
};
use crate::bootstrap::types::chain_config::CosmosChainConfig;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;
use crate::chain::types::wallet::CosmosTestWallet;

pub trait CanUseLegacyCosmosSdkChainBootstrapper: UseLegacyCosmosSdkChainBootstrapper {}

pub trait UseLegacyCosmosSdkChainBootstrapper: CanBootstrapChain {}

impl<Bootstrap, Runtime, Chain, BaseComponents> UseLegacyCosmosSdkChainBootstrapper for Bootstrap
where
    Bootstrap: HasComponents<Components = LegacyCosmosSdkBootstrapComponents<BaseComponents>>
        + HasRuntime<Runtime = Runtime>
        + HasErrorType
        + CanRaiseError<IoError>,
    BaseComponents: ProvideChainType<Bootstrap, Chain = Chain>
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
    BaseComponents::GenesisConfig: From<CosmosGenesisConfig>,
    BaseComponents::ChainConfig: From<CosmosChainConfig>,
{
}
