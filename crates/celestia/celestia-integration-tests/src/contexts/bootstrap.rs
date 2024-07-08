use alloc::sync::Arc;
use std::path::PathBuf;
use std::sync::OnceLock;

use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use eyre::Error;
use hermes_celestia_test_components::bootstrap::components::CelestiaBootstrapComponents as BaseCelestiaBootstrapComponents;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::BridgeBootstrapperComponent;
use hermes_celestia_test_components::bootstrap::traits::bridge_auth_token::BridgeAuthTokenGeneratorComponent;
use hermes_celestia_test_components::bootstrap::traits::bridge_store_dir::BridgeStoreDirGetter;
use hermes_celestia_test_components::bootstrap::traits::build_bridge_driver::BridgeDriverBuilder;
use hermes_celestia_test_components::bootstrap::traits::import_bridge_key::BridgeKeyImporterComponent;
use hermes_celestia_test_components::bootstrap::traits::init_bridge_config::BridgeConfigInitializerComponent;
use hermes_celestia_test_components::bootstrap::traits::init_bridge_data::BridgeDataInitializerComponent;
use hermes_celestia_test_components::bootstrap::traits::start_bridge::BridgeStarterComponent;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_config::BridgeConfigTypeComponent;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::ProvideBridgeDriverType;
use hermes_celestia_test_components::types::bridge_config::CelestiaBridgeConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use hermes_cosmos_integration_tests::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::types::ProvideCosmosBootstrapChainTypes;
use hermes_cosmos_integration_tests::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use hermes_cosmos_integration_tests::traits::bootstrap::compat_mode::CompatModeGetter;
use hermes_cosmos_integration_tests::traits::bootstrap::cosmos_builder::CosmosBuilderGetter;
use hermes_cosmos_integration_tests::traits::bootstrap::gas_denom::GasDenomGetter;
use hermes_cosmos_integration_tests::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::*;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilderComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::AccountPrefixGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, DenomPrefixGetter,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::ChainTypeComponent;
use hermes_test_components::driver::traits::types::chain_driver::ChainDriverTypeComponent;
use ibc_relayer::config::compat_mode::CompatMode;
use tokio::process::Child;

use crate::contexts::bridge_driver::CelestiaBridgeDriver;

#[derive(HasField)]
pub struct CelestiaBootstrap {
    pub runtime: HermesRuntime,
    pub builder: Arc<CosmosBuilder>,
    pub chain_store_dir: PathBuf,
    pub bridge_store_dir: PathBuf,
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

impl ChainStoreDirGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn chain_store_dir(bootstrap: &CelestiaBootstrap) -> &PathBuf {
        &bootstrap.chain_store_dir
    }
}

impl ChainCommandPathGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn chain_command_path(_bootstrap: &CelestiaBootstrap) -> &PathBuf {
        static CELESTIA_COMMAND_PATH: OnceLock<PathBuf> = OnceLock::new();

        CELESTIA_COMMAND_PATH.get_or_init(|| "celestia-appd".into())
    }
}

impl RandomIdFlagGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn should_randomize_identifiers(_bootstrap: &CelestiaBootstrap) -> bool {
        false
    }
}

impl CosmosGenesisConfigModifier<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn modify_genesis_config(
        _bootstrap: &CelestiaBootstrap,
        _config: &mut serde_json::Value,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl CometConfigModifier<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn modify_comet_config(
        _bootstrap: &CelestiaBootstrap,
        _comet_config: &mut toml::Value,
    ) -> Result<(), Error> {
        Ok(())
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

impl CompatModeGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn compat_mode(_bootstrap: &CelestiaBootstrap) -> Option<&CompatMode> {
        const COMPAT_MODE: CompatMode = CompatMode::V0_34;

        Some(&COMPAT_MODE)
    }
}

impl GasDenomGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn gas_denom(_bootstrap: &CelestiaBootstrap) -> &str {
        "utia"
    }
}

impl CosmosBuilderGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn cosmos_builder(bootstrap: &CelestiaBootstrap) -> &CosmosBuilder {
        &bootstrap.builder
    }
}
