use std::path::PathBuf;

use cgp::core::component::UseContext;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use hermes_cosmos_integration_tests::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use hermes_cosmos_integration_tests::impls::bootstrap::types::ProvideCosmosBootstrapChainTypes;
use hermes_cosmos_integration_tests::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use hermes_cosmos_integration_tests::traits::bootstrap::compat_mode::{
    CompatModeGetterComponent, UseCompatMode37,
};
use hermes_cosmos_integration_tests::traits::bootstrap::cosmos_builder::CosmosBuilderGetterComponent;
use hermes_cosmos_integration_tests::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk::*;
use hermes_cosmos_test_components::bootstrap::impls::generator::wallet_config::GenerateStandardWalletConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_comet_config::NoModifyCometConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_cosmos_sdk_config::NoModifyCosmosSdkConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_genesis_config::NoModifyGenesisConfig;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilderComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::AccountPrefixGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::denom::DenomPrefixGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::DynamicGasGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifierComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_cosmos_sdk_config::CosmosSdkConfigModifierComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifierComponent;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::ChainTypeComponent;
use hermes_test_components::driver::traits::types::chain_driver::ChainDriverTypeComponent;
use hermes_wasm_test_components::impls::bootstrap::build_chain_driver::BuildChainDriverAndInitWasmClient;
use hermes_wasm_test_components::impls::bootstrap::genesis_config::ModifyWasmGenesisConfig;
use hermes_wasm_test_components::impls::bootstrap::node_config::ModifyWasmNodeConfig;
use hermes_wasm_test_components::traits::bootstrap::client_byte_code::WasmClientByteCodeGetterComponent;
use hermes_wasm_test_components::traits::bootstrap::gov_authority::GovernanceProposalAuthorityGetterComponent;

/**
   A bootstrap context for bootstrapping a new Cosmos chain, and builds
   a `CosmosChainDriver`.
*/
#[derive(HasField)]
pub struct CosmosWithWasmClientBootstrap {
    pub runtime: HermesRuntime,
    pub cosmos_builder: CosmosBuilder,
    pub should_randomize_identifiers: bool,
    pub chain_store_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub account_prefix: String,
    pub staking_denom_prefix: String,
    pub transfer_denom_prefix: String,
    pub wasm_client_byte_code: Vec<u8>,
    pub governance_proposal_authority: String,
    pub dynamic_gas: Option<DynamicGasConfig>,
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
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        WalletConfigGeneratorComponent: GenerateStandardWalletConfig,
        [
            ChainTypeComponent,
            ChainDriverTypeComponent,
        ]:
            ProvideCosmosBootstrapChainTypes,
        [
            ChainStoreDirGetterComponent,
            ChainCommandPathGetterComponent,
            AccountPrefixGetterComponent,
            DenomPrefixGetterComponent,
            DynamicGasGetterComponent,
            RandomIdFlagGetterComponent,
            WasmClientByteCodeGetterComponent,
            GovernanceProposalAuthorityGetterComponent,
            CosmosBuilderGetterComponent,
        ]:
            UseContext,
        CompatModeGetterComponent:
            UseCompatMode37,
        CosmosSdkConfigModifierComponent:
            NoModifyCosmosSdkConfig,
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
