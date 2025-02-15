use core::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use cgp::core::component::UseContext;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::*;
use hermes_cosmos_test_components::bootstrap::impls::chain::build_wait::BuildAndWaitChainDriver;
use hermes_cosmos_test_components::bootstrap::impls::generator::wallet_config::GenerateStandardWalletConfig;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_cosmos_sdk_config::NoModifyCosmosSdkConfig;
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
use hermes_error::types::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use hermes_test_components::chain_driver::traits::types::chain::ChainTypeComponent;
use hermes_test_components::driver::traits::types::chain_driver::ChainDriverTypeComponent;
use tendermint_rpc::client::CompatMode;

use crate::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use crate::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use crate::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use crate::impls::bootstrap::types::ProvideCosmosBootstrapChainTypes;
use crate::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use crate::traits::bootstrap::compat_mode::CompatModeGetterComponent;
use crate::traits::bootstrap::cosmos_builder::CosmosBuilderGetterComponent;
use crate::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;

/**
   A bootstrap context for bootstrapping a new Cosmos chain, and builds
   a `CosmosChainDriver`.
*/
#[derive(Clone)]
#[cgp_context(LegacyCosmosBootstrapComponents: LegacyCosmosSdkBootstrapComponents)]
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
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
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
            CompatModeGetterComponent,
            CosmosBuilderGetterComponent,
            CometConfigModifierComponent,
            CosmosGenesisConfigModifierComponent,
        ]:
            UseContext,
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
