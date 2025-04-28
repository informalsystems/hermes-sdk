use core::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_core::test_components::chain_driver::traits::ChainTypeProviderComponent;
use hermes_core::test_components::driver::traits::ChainDriverTypeProviderComponent;
use hermes_cosmos_chain_components::types::DynamicGasConfig;
use hermes_cosmos_relayer::contexts::{CosmosBuilder, CosmosChain};
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
use hermes_error::handlers::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_error::types::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_tracing_logging_components::contexts::TracingLogger;
use tendermint_rpc::client::CompatMode;

use crate::contexts::CosmosChainDriver;
use crate::impls::{
    BuildCosmosChainDriver, BuildCosmosChainWithNodeConfig, BuildRelayerChainConfig,
};
use crate::traits::{
    ChainBuilderWithNodeConfigComponent, CompatModeGetterComponent, CosmosBuilderGetterComponent,
    RelayerChainConfigBuilderComponent,
};

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
