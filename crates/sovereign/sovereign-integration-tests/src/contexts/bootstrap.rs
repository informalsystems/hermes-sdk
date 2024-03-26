use std::path::PathBuf;

use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use hermes_celestia_integration_tests::contexts::bridge_driver::CelestiaBridgeDriver;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::ProvideBridgeDriverType;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::{DebugError, ProvideCosmosError};
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::AccountPrefixGetter;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::ProvideRollupType;
use hermes_sovereign_relayer::contexts::sovereign_rollup::SovereignRollup;
use hermes_sovereign_test_components::bootstrap::components::{
    IsSovereignBootstrapComponent, SovereignBootstrapComponents as BaseSovereignBootstrapComponents,
};
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_sovereign_test_components::bootstrap::traits::build_rollup_driver::RollupDriverBuilderComponent;
use hermes_sovereign_test_components::bootstrap::traits::rollup_command_path::RollupCommandPathGetter;
use hermes_sovereign_test_components::bootstrap::traits::rollup_store_dir::RollupStoreDirGetter;
use hermes_sovereign_test_components::bootstrap::traits::types::rollup_driver::ProvideRollupDriverType;
use hermes_test_components::chain_driver::traits::types::chain::ProvideChainType;
use hermes_test_components::driver::traits::types::chain_driver::ProvideChainDriverType;

use crate::contexts::rollup_driver::SovereignRollupDriver;
use crate::impls::build_rollup_driver::BuildSovereignRollupDriver;

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
        ErrorTypeComponent: ProvideCosmosError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeComponent: ProvideHermesRuntime,
        RollupDriverBuilderComponent: BuildSovereignRollupDriver,
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

impl ProvideRollupType<SovereignBootstrap> for SovereignBootstrapComponents {
    type Rollup = SovereignRollup;
}

impl ProvideRollupDriverType<SovereignBootstrap> for SovereignBootstrapComponents {
    type RollupDriver = SovereignRollupDriver;
}

impl RuntimeGetter<SovereignBootstrap> for SovereignBootstrapComponents {
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

pub trait CheckCanBootstrapRollup: CanBootstrapRollup {}

impl CheckCanBootstrapRollup for SovereignBootstrap {}
