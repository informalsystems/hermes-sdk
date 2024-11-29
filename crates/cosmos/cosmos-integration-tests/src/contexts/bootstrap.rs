use alloc::sync::Arc;
use cgp::core::component::UseContext;
use core::ops::Deref;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::impls::modifiers::no_modify_cosmos_sdk_config::NoModifyCosmosSdkConfig;
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::DynamicGasGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_cosmos_sdk_config::CosmosSdkConfigModifierComponent;
use std::path::PathBuf;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk::*;
use hermes_cosmos_test_components::bootstrap::impls::generator::wallet_config::GenerateStandardWalletConfig;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilderComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::AccountPrefixGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::denom::DenomPrefixGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetterComponent;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifierComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifierComponent;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::Error;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainTypeComponent, HasChainType,
};
use hermes_test_components::driver::traits::types::chain_driver::ChainDriverTypeComponent;

use crate::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use crate::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use crate::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use crate::impls::bootstrap::types::ProvideCosmosBootstrapChainTypes;
use crate::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use crate::traits::bootstrap::compat_mode::{CompatModeGetterComponent, UseCompatMode37};
use crate::traits::bootstrap::cosmos_builder::CosmosBuilderGetterComponent;
use crate::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;

/**
   A bootstrap context for bootstrapping a new Cosmos chain, and builds
   a `CosmosChainDriver`.
*/
#[derive(Clone)]
pub struct CosmosBootstrap {
    pub fields: Arc<CosmosBootstrapFields>,
}

#[derive(HasField)]
pub struct CosmosBootstrapFields {
    pub runtime: HermesRuntime,
    pub cosmos_builder: CosmosBuilder,
    pub should_randomize_identifiers: bool,
    pub chain_store_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub account_prefix: String,
    pub staking_denom_prefix: String,
    pub transfer_denom_prefix: String,
    pub genesis_config_modifier:
        Box<dyn Fn(&mut serde_json::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub comet_config_modifier:
        Box<dyn Fn(&mut toml::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub dynamic_gas: Option<DynamicGasConfig>,
}

impl CanUseCosmosSdkChainBootstrapper for CosmosBootstrap {}

pub struct CosmosBootstrapComponents;

impl HasComponents for CosmosBootstrap {
    type Components = CosmosBootstrapComponents;
}

impl Deref for CosmosBootstrap {
    type Target = CosmosBootstrapFields;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

with_cosmos_sdk_bootstrap_components! {
    delegate_components! {
        CosmosBootstrapComponents {
            @CosmosSdkBootstrapComponents: CosmosSdkBootstrapComponents,
        }
    }
}

delegate_components! {
    CosmosBootstrapComponents {
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
            CosmosBuilderGetterComponent,
            CometConfigModifierComponent,
            CosmosGenesisConfigModifierComponent,
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
            BuildCosmosChainDriver,
    }
}

pub trait CanUseCosmosBootstrap: HasChainType<Chain = CosmosChain> {}

impl CanUseCosmosBootstrap for CosmosBootstrap {}
