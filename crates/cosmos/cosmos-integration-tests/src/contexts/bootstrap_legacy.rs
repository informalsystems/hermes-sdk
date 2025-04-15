use core::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::DynamicGasConfig;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::components::LegacyCosmosSdkBootstrapComponents;
use hermes_cosmos_test_components::bootstrap::impls::{
    BuildAndWaitChainDriver, GenerateStandardWalletConfig, NoModifyCosmosSdkConfig,
};
use hermes_cosmos_test_components::bootstrap::traits::{
    AccountPrefixGetterComponent, ChainCommandPathGetterComponent, ChainDriverBuilderComponent,
    ChainStoreDirGetterComponent, CometConfigModifierComponent,
    CosmosGenesisConfigModifierComponent, CosmosSdkConfigModifierComponent, DenomForStaking,
    DenomForTransfer, DenomPrefixGetterComponent, DynamicGasGetterComponent,
    RandomIdFlagGetterComponent, WalletConfigGeneratorComponent,
};
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_error::types::Error;
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_test_components::chain_driver::traits::ChainTypeProviderComponent;
use hermes_test_components::driver::traits::ChainDriverTypeProviderComponent;
use hermes_tracing_logging_components::contexts::logger::TracingLogger;
use tendermint_rpc::client::CompatMode;

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use crate::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use crate::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use crate::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use crate::traits::bootstrap::compat_mode::CompatModeGetterComponent;
use crate::traits::bootstrap::cosmos_builder::CosmosBuilderGetterComponent;
use crate::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;

/**
   A bootstrap context for bootstrapping a new Cosmos chain, and builds
   a `CosmosChainDriver`.
*/
#[cgp_context(LegacyCosmosBootstrapComponents: LegacyCosmosSdkBootstrapComponents)]
#[derive(Clone)]
pub struct LegacyCosmosBootstrap {
    pub fields: Arc<LegacyCosmosBootstrapFields>,
}

#[derive(HasField)]
pub struct LegacyCosmosBootstrapFields {
    pub runtime: HermesRuntime,
    pub cosmos_builder: CosmosBuilder,
    pub should_randomize_identifiers: bool,
    pub chain_store_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub account_prefix: String,
    pub staking_denom_prefix: String,
    pub transfer_denom_prefix: String,
    pub compat_mode: Option<CompatMode>,
    pub genesis_config_modifier:
        Box<dyn Fn(&mut serde_json::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub comet_config_modifier:
        Box<dyn Fn(&mut toml::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub dynamic_gas: Option<DynamicGasConfig>,
}

impl Deref for LegacyCosmosBootstrap {
    type Target = LegacyCosmosBootstrapFields;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

delegate_components! {
    LegacyCosmosBootstrapComponents {
        ErrorTypeProviderComponent:
            UseHermesError,
        ErrorRaiserComponent:
            DebugError,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        LoggerComponent:
            TracingLogger,
        WalletConfigGeneratorComponent:
            GenerateStandardWalletConfig,
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
        CompatModeGetterComponent:
            UseField<symbol!("compat_mode")>,
        CosmosBuilderGetterComponent:
            UseField<symbol!("cosmos_builder")>,
        CosmosGenesisConfigModifierComponent:
            UseField<symbol!("genesis_config_modifier")>,
        CometConfigModifierComponent:
            UseField<symbol!("comet_config_modifier")>,
        CosmosSdkConfigModifierComponent:
            NoModifyCosmosSdkConfig,
        RelayerChainConfigBuilderComponent:
            BuildRelayerChainConfig,
        ChainBuilderWithNodeConfigComponent:
            BuildCosmosChainWithNodeConfig,
        ChainDriverBuilderComponent:
            BuildAndWaitChainDriver<BuildCosmosChainDriver>,
    }
}
