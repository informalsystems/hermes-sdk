use std::path::PathBuf;
use std::sync::OnceLock;

use cgp::core::component::UseContext;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use eyre::Error;
use hermes_celestia_test_components::bootstrap::components::CelestiaBootstrapComponents as BaseCelestiaBootstrapComponents;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::BridgeBootstrapperComponent;
use hermes_celestia_test_components::bootstrap::traits::bridge_auth_token::{
    BridgeAuthTokenGeneratorComponent, CanGenerateBridgeAuthToken,
};
use hermes_celestia_test_components::bootstrap::traits::bridge_store_dir::{
    BridgeStoreDirGetter, HasBridgeStoreDir,
};
use hermes_celestia_test_components::bootstrap::traits::build_bridge_driver::{
    BridgeDriverBuilder, CanBuildBridgeDriver,
};
use hermes_celestia_test_components::bootstrap::traits::import_bridge_key::{
    BridgeKeyImporterComponent, CanImportBridgeKey,
};
use hermes_celestia_test_components::bootstrap::traits::init_bridge_config::{
    BridgeConfigInitializerComponent, CanInitBridgeConfig,
};
use hermes_celestia_test_components::bootstrap::traits::init_bridge_data::{
    BridgeDataInitializerComponent, CanInitBridgeData,
};
use hermes_celestia_test_components::bootstrap::traits::start_bridge::{
    BridgeStarterComponent, CanStartBridge,
};
use hermes_celestia_test_components::bootstrap::traits::types::bridge_config::{
    BridgeConfigTypeComponent, HasBridgeConfigType,
};
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::ProvideBridgeDriverType;
use hermes_celestia_test_components::types::bridge_config::CelestiaBridgeConfig;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use hermes_cosmos_integration_tests::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::types::ProvideCosmosBootstrapChainTypes;
use hermes_cosmos_integration_tests::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use hermes_cosmos_integration_tests::traits::bootstrap::compat_mode::{
    CompatModeGetterComponent, UseCompatMode34,
};
use hermes_cosmos_integration_tests::traits::bootstrap::cosmos_builder::CosmosBuilderGetterComponent;
use hermes_cosmos_integration_tests::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::*;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_comet_config::NoModifyCometConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_cosmos_sdk_config::NoModifyCosmosSdkConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_genesis_config::NoModifyGenesisConfig;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilderComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::AccountPrefixGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, DenomPrefixGetter,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::DynamicGasGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::{
    RandomIdFlagGetterComponent, ReturnRandomIdFlag,
};
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifierComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_cosmos_sdk_config::CosmosSdkConfigModifierComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifierComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainTypeComponent, HasChainType,
};
use hermes_test_components::driver::traits::types::chain_driver::{
    ChainDriverTypeComponent, HasChainDriverType,
};
use tokio::process::Child;

use crate::contexts::bridge_driver::CelestiaBridgeDriver;

#[derive(HasField)]
pub struct CelestiaBootstrap {
    pub runtime: HermesRuntime,
    pub cosmos_builder: CosmosBuilder,
    pub chain_store_dir: PathBuf,
    pub bridge_store_dir: PathBuf,
    pub dynamic_gas: Option<DynamicGasConfig>,
}

impl CanUseLegacyCosmosSdkChainBootstrapper for CelestiaBootstrap {}

pub struct CelestiaBootstrapComponents;

impl HasComponents for CelestiaBootstrap {
    type Components = CelestiaBootstrapComponents;
}

with_legacy_cosmos_sdk_bootstrap_components! {
    delegate_components! {
        CelestiaBootstrapComponents {
            @LegacyCosmosSdkBootstrapComponents: LegacyCosmosSdkBootstrapComponents,
        }
    }
}

delegate_components! {
    CelestiaBootstrapComponents {
        [
            WalletConfigGeneratorComponent,
            BridgeBootstrapperComponent,
            BridgeDataInitializerComponent,
            BridgeKeyImporterComponent,
            BridgeConfigTypeComponent,
            BridgeConfigInitializerComponent,
            BridgeAuthTokenGeneratorComponent,
            BridgeStarterComponent,
        ]:
            BaseCelestiaBootstrapComponents,
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        [
            ChainTypeComponent,
            ChainDriverTypeComponent,
        ]:
            ProvideCosmosBootstrapChainTypes,
        [
            ChainStoreDirGetterComponent,
            CosmosBuilderGetterComponent,
            DynamicGasGetterComponent,
        ]:
            UseContext,
        RandomIdFlagGetterComponent:
            ReturnRandomIdFlag<false>,
        CompatModeGetterComponent:
            UseCompatMode34,
        CosmosGenesisConfigModifierComponent:
            NoModifyGenesisConfig,
        CometConfigModifierComponent:
            NoModifyCometConfig,
        CosmosSdkConfigModifierComponent:
            NoModifyCosmosSdkConfig,
        RelayerChainConfigBuilderComponent:
            BuildRelayerChainConfig,
        ChainBuilderWithNodeConfigComponent:
            BuildCosmosChainWithNodeConfig,
        ChainDriverBuilderComponent:
            BuildCosmosChainDriver,
    }
}

impl ProvideBridgeDriverType<CelestiaBootstrap> for CelestiaBootstrapComponents {
    type BridgeDriver = CelestiaBridgeDriver;
}

impl BridgeStoreDirGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn bridge_store_dir(bootstrap: &CelestiaBootstrap) -> &PathBuf {
        &bootstrap.bridge_store_dir
    }
}

impl BridgeDriverBuilder<CelestiaBootstrap> for CelestiaBootstrapComponents {
    async fn build_bridge_driver(
        _bootstrap: &CelestiaBootstrap,
        bridge_config: CelestiaBridgeConfig,
        bridge_auth_token: String,
        bridge_process: Child,
    ) -> Result<CelestiaBridgeDriver, Error> {
        Ok(CelestiaBridgeDriver {
            bridge_config,
            bridge_auth_token,
            bridge_process,
        })
    }
}

impl ChainCommandPathGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn chain_command_path(_bootstrap: &CelestiaBootstrap) -> &PathBuf {
        static CELESTIA_COMMAND_PATH: OnceLock<PathBuf> = OnceLock::new();

        CELESTIA_COMMAND_PATH.get_or_init(|| "celestia-appd".into())
    }
}

impl DenomPrefixGetter<CelestiaBootstrap, DenomForStaking> for CelestiaBootstrapComponents {
    fn denom_prefix(_bootstrap: &CelestiaBootstrap, _label: DenomForStaking) -> &str {
        "utia"
    }
}

impl DenomPrefixGetter<CelestiaBootstrap, DenomForTransfer> for CelestiaBootstrapComponents {
    fn denom_prefix(_bootstrap: &CelestiaBootstrap, _label: DenomForTransfer) -> &str {
        "coin"
    }
}

impl AccountPrefixGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn account_prefix(_bootstrap: &CelestiaBootstrap) -> &str {
        "celestia"
    }
}

use core::str::Utf8Error;
use std::io::Error as IoError;
use std::process::ExitStatus;

use hermes_async_runtime_components::channel::types::ChannelClosedError;
use hermes_tokio_runtime_components::impls::os::child_process::PrematureChildProcessExitError;
use hermes_tokio_runtime_components::impls::os::exec_command::{
    CommandNotFound, ExecCommandFailure,
};

trait CanBootstrapCelestia: HasChainType<Chain = CosmosChain>
        + HasChainDriverType<ChainDriver = CosmosChainDriver>
        + HasRuntime<Runtime = HermesRuntime>
        + HasBridgeStoreDir
        + CanInitBridgeData
        + CanImportBridgeKey
        + CanGenerateBridgeAuthToken<BridgeDriver = CelestiaBridgeDriver>
        + CanStartBridge
        + CanBuildBridgeDriver
        + HasBridgeStoreDir
        + CanInitBridgeData
        + CanImportBridgeKey
        + CanGenerateBridgeAuthToken
        + CanStartBridge
        + CanBuildBridgeDriver
        + HasBridgeConfigType
        //+ CanInitBridgeConfig // This fails
        // These CanRaiseError are taken from CanRaiseError<Runtime::Error> for HermesRuntime
        // from this file crates/runtime/runtime/src/impls/runtime/error.rs
        + CanRaiseError<PrematureChildProcessExitError> // This fails
        + CanRaiseError<ExitStatus>
        + CanRaiseError<IoError>
        + CanRaiseError<Utf8Error>
        + CanRaiseError<ChannelClosedError> // This fails
        + CanRaiseError<ExecCommandFailure> // This fails
        + CanRaiseError<CommandNotFound> // This fails

        + CanRaiseError<toml::de::Error>
        + CanRaiseError<toml::ser::Error>
        + CanRaiseError<&'static str>
    + CanRaiseError<toml::de::Error>
    + CanRaiseError<toml::ser::Error>
    + CanRaiseError<&'static str>
{

}

impl CanBootstrapCelestia for CelestiaBootstrap {}
