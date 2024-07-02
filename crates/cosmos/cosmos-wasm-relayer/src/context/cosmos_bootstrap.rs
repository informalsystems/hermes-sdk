use alloc::sync::Arc;
use std::path::PathBuf;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use hermes_cosmos_integration_tests::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::types::ProvideCosmosBootstrapChainTypes;
use hermes_cosmos_integration_tests::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use hermes_cosmos_integration_tests::traits::bootstrap::compat_mode::CompatModeGetter;
use hermes_cosmos_integration_tests::traits::bootstrap::cosmos_builder::CosmosBuilderGetter;
use hermes_cosmos_integration_tests::traits::bootstrap::gas_denom::GasDenomGetter;
use hermes_cosmos_integration_tests::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::{DebugError, ProvideCosmosError};
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk::*;
use hermes_cosmos_test_components::bootstrap::impls::generator::wallet_config::GenerateStandardWalletConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_comet_config::NoModifyCometConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_genesis_config::NoModifyGenesisConfig;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilderComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::AccountPrefixGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, DenomPrefixGetter,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifierComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifierComponent;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use hermes_test_components::chain_driver::traits::types::chain::ChainTypeComponent;
use hermes_test_components::driver::traits::types::chain_driver::ChainDriverTypeComponent;
use hermes_wasm_test_components::impls::bootstrap::build_chain_driver::BuildChainDriverAndInitWasmClient;
use hermes_wasm_test_components::impls::bootstrap::genesis_config::ModifyWasmGenesisConfig;
use hermes_wasm_test_components::impls::bootstrap::node_config::ModifyWasmNodeConfig;
use hermes_wasm_test_components::traits::bootstrap::client_code_path::WasmClientCodePathGetter;
use ibc_relayer::config::compat_mode::CompatMode;

/**
   A bootstrap context for bootstrapping a new Cosmos chain, and builds
   a `CosmosChainDriver`.
*/
pub struct CosmosWithWasmClientBootstrap {
    pub runtime: HermesRuntime,
    pub builder: Arc<CosmosBuilder>,
    pub should_randomize_identifiers: bool,
    pub chain_store_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub account_prefix: String,
    pub staking_denom: String,
    pub transfer_denom: String,
    pub wasm_client_code_path: PathBuf,
}

impl CanUseCosmosSdkChainBootstrapper for CosmosWithWasmClientBootstrap {}

pub struct CosmosWithWasmClientBootstrapComponents;

impl HasComponents for CosmosWithWasmClientBootstrap {
    type Components = CosmosWithWasmClientBootstrapComponents;
}

with_cosmos_sdk_bootstrap_components! {
    delegate_components! {
        CosmosWithWasmClientBootstrapComponents {
            @CosmosSdkBootstrapComponents: CosmosSdkBootstrapComponents,
        }
    }
}

delegate_components! {
    CosmosWithWasmClientBootstrapComponents {
        ErrorTypeComponent: ProvideCosmosError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeComponent: ProvideHermesRuntime,
        WalletConfigGeneratorComponent: GenerateStandardWalletConfig,
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
            BuildChainDriverAndInitWasmClient<BuildCosmosChainDriver>,
        CosmosGenesisConfigModifierComponent:
            ModifyWasmGenesisConfig<NoModifyGenesisConfig>,
        CometConfigModifierComponent:
            ModifyWasmNodeConfig<NoModifyCometConfig>,
    }
}

impl RuntimeGetter<CosmosWithWasmClientBootstrap> for CosmosWithWasmClientBootstrapComponents {
    fn runtime(bootstrap: &CosmosWithWasmClientBootstrap) -> &HermesRuntime {
        &bootstrap.runtime
    }
}

impl ChainStoreDirGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn chain_store_dir(bootstrap: &CosmosWithWasmClientBootstrap) -> &PathBuf {
        &bootstrap.chain_store_dir
    }
}

impl ChainCommandPathGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn chain_command_path(bootstrap: &CosmosWithWasmClientBootstrap) -> &PathBuf {
        &bootstrap.chain_command_path
    }
}

impl RandomIdFlagGetter<CosmosWithWasmClientBootstrap> for CosmosWithWasmClientBootstrapComponents {
    fn should_randomize_identifiers(bootstrap: &CosmosWithWasmClientBootstrap) -> bool {
        bootstrap.should_randomize_identifiers
    }
}

impl DenomPrefixGetter<CosmosWithWasmClientBootstrap, DenomForStaking>
    for CosmosWithWasmClientBootstrapComponents
{
    fn denom_prefix(bootstrap: &CosmosWithWasmClientBootstrap, _label: DenomForStaking) -> &str {
        &bootstrap.staking_denom
    }
}

impl DenomPrefixGetter<CosmosWithWasmClientBootstrap, DenomForTransfer>
    for CosmosWithWasmClientBootstrapComponents
{
    fn denom_prefix(bootstrap: &CosmosWithWasmClientBootstrap, _label: DenomForTransfer) -> &str {
        &bootstrap.transfer_denom
    }
}

impl AccountPrefixGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn account_prefix(bootstrap: &CosmosWithWasmClientBootstrap) -> &str {
        &bootstrap.account_prefix
    }
}

impl CompatModeGetter<CosmosWithWasmClientBootstrap> for CosmosWithWasmClientBootstrapComponents {
    fn compat_mode(_bootstrap: &CosmosWithWasmClientBootstrap) -> Option<&CompatMode> {
        const COMPAT_MODE: CompatMode = CompatMode::V0_37;

        Some(&COMPAT_MODE)
    }
}

impl GasDenomGetter<CosmosWithWasmClientBootstrap> for CosmosWithWasmClientBootstrapComponents {
    fn gas_denom(bootstrap: &CosmosWithWasmClientBootstrap) -> &str {
        &bootstrap.staking_denom
    }
}

impl CosmosBuilderGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn cosmos_builder(bootstrap: &CosmosWithWasmClientBootstrap) -> &CosmosBuilder {
        &bootstrap.builder
    }
}

impl WasmClientCodePathGetter<CosmosWithWasmClientBootstrap>
    for CosmosWithWasmClientBootstrapComponents
{
    fn wasm_client_code_path(bootstrap: &CosmosWithWasmClientBootstrap) -> &PathBuf {
        &bootstrap.wasm_client_code_path
    }
}
