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
use hermes_cosmos_integration_tests::contexts::CosmosChainDriver;
use hermes_cosmos_integration_tests::impls::{
    BuildCosmosChainDriver, BuildCosmosChainWithNodeConfig, BuildRelayerChainConfig,
};
use hermes_cosmos_integration_tests::traits::{
    ChainBuilderWithNodeConfigComponent, CompatModeGetterComponent, CosmosBuilderGetterComponent,
    RelayerChainConfigBuilderComponent, UseCompatMode34,
};
use hermes_cosmos_relayer::contexts::{CosmosBuilder, CosmosChain};
use hermes_cosmos_test_components::bootstrap::components::LegacyCosmosSdkBootstrapComponents;
use hermes_cosmos_test_components::bootstrap::impls::{
    NoModifyCometConfig, NoModifyCosmosSdkConfig, NoModifyGenesisConfig,
};
use hermes_cosmos_test_components::bootstrap::traits::{
    AccountPrefixGetter, AccountPrefixGetterComponent, ChainCommandPathGetter,
    ChainCommandPathGetterComponent, ChainDriverBuilderComponent, ChainStoreDirGetterComponent,
    CometConfigModifierComponent, CosmosGenesisConfigModifierComponent,
    CosmosSdkConfigModifierComponent, DenomForStaking, DenomForTransfer, DenomPrefixGetter,
    DenomPrefixGetterComponent, DynamicGasGetterComponent, RandomIdFlagGetterComponent,
    UseRandomIdFlag, WalletConfigGeneratorComponent,
};
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_error::types::HermesError;
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::{RuntimeGetterComponent, RuntimeTypeProviderComponent};
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
