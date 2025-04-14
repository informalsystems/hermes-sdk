use core::marker::PhantomData;
use std::path::PathBuf;
use std::sync::OnceLock;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_celestia_test_components::bootstrap::components::CelestiaBootstrapComponents as BaseCelestiaBootstrapComponents;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::BridgeBootstrapperComponent;
use hermes_celestia_test_components::bootstrap::traits::bridge_auth_token::BridgeAuthTokenGeneratorComponent;
use hermes_celestia_test_components::bootstrap::traits::bridge_store_dir::{
    BridgeStoreDirGetter, BridgeStoreDirGetterComponent,
};
use hermes_celestia_test_components::bootstrap::traits::build_bridge_driver::{
    BridgeDriverBuilder, BridgeDriverBuilderComponent,
};
use hermes_celestia_test_components::bootstrap::traits::import_bridge_key::BridgeKeyImporterComponent;
use hermes_celestia_test_components::bootstrap::traits::init_bridge_config::BridgeConfigInitializerComponent;
use hermes_celestia_test_components::bootstrap::traits::init_bridge_data::BridgeDataInitializerComponent;
use hermes_celestia_test_components::bootstrap::traits::start_bridge::BridgeStarterComponent;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_config::BridgeConfigTypeComponent;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::{
    BridgeDriverTypeComponent, ProvideBridgeDriverType,
};
use hermes_celestia_test_components::types::bridge_config::CelestiaBridgeConfig;
use hermes_cosmos_chain_components::types::DynamicGasConfig;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use hermes_cosmos_integration_tests::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
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
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::{
    AccountPrefixGetter, AccountPrefixGetterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::{
    ChainCommandPathGetter, ChainCommandPathGetterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, DenomPrefixGetter, DenomPrefixGetterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::DynamicGasGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::{
    RandomIdFlagGetterComponent, UseRandomIdFlag,
};
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifierComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_cosmos_sdk_config::CosmosSdkConfigModifierComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifierComponent;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_error::types::HermesError;
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_test_components::bootstrap::traits::ChainBootstrapperComponent;
use hermes_test_components::chain_driver::traits::ChainTypeProviderComponent;
use hermes_test_components::driver::traits::ChainDriverTypeProviderComponent;
use hermes_tracing_logging_components::contexts::logger::TracingLogger;
use tokio::process::Child;

use crate::contexts::bridge_driver::CelestiaBridgeDriver;

#[cgp_context(CelestiaBootstrapComponents: LegacyCosmosSdkBootstrapComponents)]
#[derive(HasField)]
pub struct CelestiaBootstrap {
    pub runtime: HermesRuntime,
    pub cosmos_builder: CosmosBuilder,
    pub chain_store_dir: PathBuf,
    pub bridge_store_dir: PathBuf,
    pub dynamic_gas: Option<DynamicGasConfig>,
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
            BaseCelestiaBootstrapComponents::Provider,
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        LoggerComponent:
            TracingLogger,
        ChainTypeProviderComponent:
            UseType<CosmosChain>,
        ChainDriverTypeProviderComponent:
            UseType<CosmosChainDriver>,
        ChainStoreDirGetterComponent:
            UseField<symbol!("chain_store_dir")>,
        CosmosBuilderGetterComponent:
            UseField<symbol!("cosmos_builder")>,
        DynamicGasGetterComponent:
            UseField<symbol!("dynamic_gas")>,
        RandomIdFlagGetterComponent:
            UseRandomIdFlag<false>,
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

#[cgp_provider(BridgeDriverTypeComponent)]
impl ProvideBridgeDriverType<CelestiaBootstrap> for CelestiaBootstrapComponents {
    type BridgeDriver = CelestiaBridgeDriver;
}

#[cgp_provider(BridgeStoreDirGetterComponent)]
impl BridgeStoreDirGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn bridge_store_dir(bootstrap: &CelestiaBootstrap) -> &PathBuf {
        &bootstrap.bridge_store_dir
    }
}

#[cgp_provider(BridgeDriverBuilderComponent)]
impl BridgeDriverBuilder<CelestiaBootstrap> for CelestiaBootstrapComponents {
    async fn build_bridge_driver(
        _bootstrap: &CelestiaBootstrap,
        bridge_config: CelestiaBridgeConfig,
        bridge_auth_token: String,
        bridge_process: Child,
    ) -> Result<CelestiaBridgeDriver, HermesError> {
        Ok(CelestiaBridgeDriver {
            bridge_config,
            bridge_auth_token,
            bridge_process,
        })
    }
}

#[cgp_provider(ChainCommandPathGetterComponent)]
impl ChainCommandPathGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn chain_command_path(_bootstrap: &CelestiaBootstrap) -> &PathBuf {
        static CELESTIA_COMMAND_PATH: OnceLock<PathBuf> = OnceLock::new();

        CELESTIA_COMMAND_PATH.get_or_init(|| "celestia-appd".into())
    }
}

#[cgp_provider(DenomPrefixGetterComponent<DenomForStaking>)]
impl DenomPrefixGetter<CelestiaBootstrap, DenomForStaking> for CelestiaBootstrapComponents {
    fn denom_prefix(_bootstrap: &CelestiaBootstrap, _label: PhantomData<DenomForStaking>) -> &str {
        "utia"
    }
}

#[cgp_provider(DenomPrefixGetterComponent<DenomForTransfer>)]
impl DenomPrefixGetter<CelestiaBootstrap, DenomForTransfer> for CelestiaBootstrapComponents {
    fn denom_prefix(_bootstrap: &CelestiaBootstrap, _label: PhantomData<DenomForTransfer>) -> &str {
        "coin"
    }
}

#[cgp_provider(AccountPrefixGetterComponent)]
impl AccountPrefixGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn account_prefix(_bootstrap: &CelestiaBootstrap) -> &str {
        "celestia"
    }
}

check_components! {
    CanUseCelestiaBootstrap for CelestiaBootstrap {
        ChainBootstrapperComponent,
    }
}
