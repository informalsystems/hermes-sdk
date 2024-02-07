use std::path::PathBuf;

use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use eyre::Error;
use hermes_celestia_integration_tests::contexts::bridge_driver::CelestiaBridgeDriver;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::ProvideBridgeDriverType;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_test_components::bootstrap::components::IsSovereignBootstrapComponent;
use hermes_sovereign_test_components::bootstrap::components::SovereignBootstrapComponents as BaseSovereignBootstrapComponents;
use hermes_sovereign_test_components::bootstrap::traits::account_prefix::AccountPrefixGetter;
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_sovereign_test_components::bootstrap::traits::build_rollup_driver::RollupDriverBuilder;
use hermes_sovereign_test_components::bootstrap::traits::rollup_command_path::RollupCommandPathGetter;
use hermes_sovereign_test_components::bootstrap::traits::rollup_store_dir::RollupStoreDirGetter;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_driver::ProvideRollupDriverType;
use hermes_sovereign_test_components::types::rollup_genesis_config::SovereignGenesisConfig;
use hermes_sovereign_test_components::types::rollup_node_config::SovereignRollupNodeConfig;
use hermes_test_components::chain_driver::traits::types::chain::ProvideChainType;
use hermes_test_components::chain_driver::traits::types::wallet::HasWalletType;
use hermes_test_components::driver::traits::types::chain_driver::ProvideChainDriverType;
use tokio::process::Child;

use crate::contexts::rollup_driver::SovereignRollupDriver;

pub struct SovereignBootstrap {
    pub runtime: HermesRuntime,
    pub rollup_store_dir: PathBuf,
    pub rollup_command_path: PathBuf,
    pub account_prefix: String,
}

pub struct SovereignBootstrapComponents;

delegate_all!(
    IsSovereignBootstrapComponent,
    BaseSovereignBootstrapComponents,
    SovereignBootstrapComponents,
);

delegate_components! {
    SovereignBootstrapComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideTokioRuntimeType,
    }
}

impl HasComponents for SovereignBootstrap {
    type Components = SovereignBootstrapComponents;
}

impl ProvideChainType<SovereignBootstrap> for SovereignBootstrapComponents {
    type Chain = CosmosChain;
}

impl ProvideChainDriverType<SovereignBootstrap> for SovereignBootstrapComponents {
    type ChainDriver = CosmosChainDriver;
}

impl ProvideBridgeDriverType<SovereignBootstrap> for SovereignBootstrapComponents {
    type BridgeDriver = CelestiaBridgeDriver;
}

impl ProvideRollupDriverType<SovereignBootstrap> for SovereignBootstrapComponents {
    type RollupDriver = SovereignRollupDriver;
}

impl ProvideRuntime<SovereignBootstrap> for SovereignBootstrapComponents {
    fn runtime(bootstrap: &SovereignBootstrap) -> &HermesRuntime {
        &bootstrap.runtime
    }
}

impl RollupStoreDirGetter<SovereignBootstrap> for SovereignBootstrapComponents {
    fn rollup_store_dir(bootstrap: &SovereignBootstrap) -> &PathBuf {
        &bootstrap.rollup_store_dir
    }
}

impl AccountPrefixGetter<SovereignBootstrap> for SovereignBootstrapComponents {
    fn account_prefix(bootstrap: &SovereignBootstrap) -> &str {
        &bootstrap.account_prefix
    }
}

impl RollupCommandPathGetter<SovereignBootstrap> for SovereignBootstrapComponents {
    fn rollup_command_path(bootstrap: &SovereignBootstrap) -> &PathBuf {
        &bootstrap.rollup_command_path
    }
}

impl RollupDriverBuilder<SovereignBootstrap> for SovereignBootstrapComponents {
    async fn build_rollup_driver(
        _bootstrap: &SovereignBootstrap,
        _rollup_node_config: SovereignRollupNodeConfig,
        _genesis_config: SovereignGenesisConfig,
        _rollup_process: Child,
    ) -> Result<SovereignRollupDriver, Error> {
        Ok(SovereignRollupDriver {})
    }
}

pub trait CheckCanBootstrapRollup: CanBootstrapRollup
where
    Self::RollupDriver: HasWalletType,
{
}

impl CheckCanBootstrapRollup for SovereignBootstrap {}
