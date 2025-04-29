use std::path::PathBuf;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_core::test_components::chain_driver::traits::ChainTypeProviderComponent;
use hermes_core::test_components::driver::traits::ChainDriverTypeProviderComponent;
use hermes_cosmos_core::chain_components::types::DynamicGasConfig;
use hermes_cosmos_core::test_components::bootstrap::components::CosmosSdkBootstrapComponents;
use hermes_cosmos_core::test_components::bootstrap::impls::{
    BuildAndWaitChainDriver, GenerateStandardWalletConfig, NoModifyCometConfig,
    NoModifyCosmosSdkConfig, NoModifyGenesisConfig,
};
use hermes_cosmos_core::test_components::bootstrap::traits::{
    AccountPrefixGetterComponent, ChainCommandPathGetterComponent, ChainDriverBuilderComponent,
    ChainStoreDirGetterComponent, CometConfigModifierComponent,
    CosmosGenesisConfigModifierComponent, CosmosSdkConfigModifierComponent, DenomForStaking,
    DenomForTransfer, DenomPrefixGetterComponent, DynamicGasGetterComponent,
    RandomIdFlagGetterComponent, WalletConfigGeneratorComponent,
};
use hermes_cosmos_core::tracing_logging_components::contexts::TracingLogger;
use hermes_cosmos_core::wasm_test_components::impls::bootstrap::{
    BuildChainDriverAndInitWasmClient, ModifyWasmGenesisConfig, ModifyWasmNodeConfig,
};
use hermes_cosmos_core::wasm_test_components::traits::bootstrap::{
    GovernanceProposalAuthorityGetterComponent, WasmClientByteCodeGetterComponent,
};
use hermes_cosmos_integration_tests::contexts::CosmosChainDriver;
use hermes_cosmos_integration_tests::impls::{
    BuildCosmosChainDriver, BuildCosmosChainWithNodeConfig, BuildRelayerChainConfig,
};
use hermes_cosmos_integration_tests::traits::{
    ChainBuilderWithNodeConfigComponent, CompatModeGetterComponent, CosmosBuilderGetterComponent,
    RelayerChainConfigBuilderComponent, UseCompatMode37,
};
use hermes_cosmos_relayer::contexts::{CosmosBuilder, CosmosChain};
use hermes_error::handlers::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_prelude::*;
use hermes_runtime::types::runtime::HermesRuntime;

/**
   A bootstrap context for bootstrapping a new Cosmos chain, and builds
   a `CosmosChainDriver`.
*/
#[cgp_context(CosmosWithWasmClientBootstrapComponents: CosmosSdkBootstrapComponents)]
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

delegate_components! {
    CosmosWithWasmClientBootstrapComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        LoggerComponent:
            TracingLogger,
        WalletConfigGeneratorComponent: GenerateStandardWalletConfig,
        ChainTypeProviderComponent:
            UseType<CosmosChain>,
        ChainDriverTypeProviderComponent:
            UseType<CosmosChainDriver>,
        ChainStoreDirGetterComponent:
            UseField<symbol!("chain_store_dir")>,
        ChainCommandPathGetterComponent:
            UseField<symbol!("chain_command_path")>,
        AccountPrefixGetterComponent:
            UseField<symbol!("account_prefix")>,
        DenomPrefixGetterComponent<DenomForStaking>:
            UseField<symbol!("staking_denom_prefix")>,
        DenomPrefixGetterComponent<DenomForTransfer>:
            UseField<symbol!("transfer_denom_prefix")>,
        DynamicGasGetterComponent:
            UseField<symbol!("dynamic_gas")>,
        RandomIdFlagGetterComponent:
            UseField<symbol!("should_randomize_identifiers")>,
        CosmosBuilderGetterComponent:
            UseField<symbol!("cosmos_builder")>,
        WasmClientByteCodeGetterComponent:
            UseField<symbol!("wasm_client_byte_code")>,
        GovernanceProposalAuthorityGetterComponent:
            UseField<symbol!("governance_proposal_authority")>,
        CompatModeGetterComponent:
            UseCompatMode37,
        CosmosSdkConfigModifierComponent:
            NoModifyCosmosSdkConfig,
        RelayerChainConfigBuilderComponent:
            BuildRelayerChainConfig,
        ChainBuilderWithNodeConfigComponent:
            BuildCosmosChainWithNodeConfig,
        ChainDriverBuilderComponent:
            BuildChainDriverAndInitWasmClient<BuildAndWaitChainDriver<BuildCosmosChainDriver>>,
        CosmosGenesisConfigModifierComponent:
            ModifyWasmGenesisConfig<NoModifyGenesisConfig>,
        CometConfigModifierComponent:
            ModifyWasmNodeConfig<NoModifyCometConfig>,
    }
}
