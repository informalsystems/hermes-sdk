use alloc::sync::Arc;
use std::path::PathBuf;

use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::*;
use hermes_cosmos_test_components::bootstrap::impls::generator::wallet_config::GenerateStandardWalletConfig;
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
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::Error;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    GetRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::ChainTypeComponent;
use hermes_test_components::driver::traits::types::chain_driver::ChainDriverTypeComponent;
use ibc_relayer::config::compat_mode::CompatMode;

use crate::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use crate::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use crate::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use crate::impls::bootstrap::types::ProvideCosmosBootstrapChainTypes;
use crate::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use crate::traits::bootstrap::compat_mode::CompatModeGetter;
use crate::traits::bootstrap::cosmos_builder::CosmosBuilderGetter;
use crate::traits::bootstrap::gas_denom::GasDenomGetter;
use crate::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;

/**
   A bootstrap context for bootstrapping a new Cosmos chain, and builds
   a `CosmosChainDriver`.
*/
#[derive(HasField)]
pub struct LegacyCosmosBootstrap {
    pub runtime: HermesRuntime,
    pub builder: Arc<CosmosBuilder>,
    pub should_randomize_identifiers: bool,
    pub chain_store_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub account_prefix: String,
    pub staking_denom: String,
    pub transfer_denom: String,
    pub compat_mode: Option<CompatMode>,
    pub genesis_config_modifier:
        Box<dyn Fn(&mut serde_json::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub comet_config_modifier:
        Box<dyn Fn(&mut toml::Value) -> Result<(), Error> + Send + Sync + 'static>,
}

impl CanUseLegacyCosmosSdkChainBootstrapper for LegacyCosmosBootstrap {}

pub struct LegacyCosmosBootstrapComponents;

impl HasComponents for LegacyCosmosBootstrap {
    type Components = LegacyCosmosBootstrapComponents;
}

with_legacy_cosmos_sdk_bootstrap_components! {
    delegate_components! {
        LegacyCosmosBootstrapComponents {
            @LegacyCosmosSdkBootstrapComponents: LegacyCosmosSdkBootstrapComponents,
        }
    }
}

delegate_components! {
    LegacyCosmosBootstrapComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeComponent: ProvideHermesRuntime,
        RuntimeGetterComponent:
            GetRuntimeField<symbol!("runtime")>,
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
            BuildCosmosChainDriver,
    }
}

impl ChainStoreDirGetter<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn chain_store_dir(bootstrap: &LegacyCosmosBootstrap) -> &PathBuf {
        &bootstrap.chain_store_dir
    }
}

impl ChainCommandPathGetter<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn chain_command_path(bootstrap: &LegacyCosmosBootstrap) -> &PathBuf {
        &bootstrap.chain_command_path
    }
}

impl RandomIdFlagGetter<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn should_randomize_identifiers(bootstrap: &LegacyCosmosBootstrap) -> bool {
        bootstrap.should_randomize_identifiers
    }
}

impl CosmosGenesisConfigModifier<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn modify_genesis_config(
        bootstrap: &LegacyCosmosBootstrap,
        config: &mut serde_json::Value,
    ) -> Result<(), Error> {
        (bootstrap.genesis_config_modifier)(config)
    }
}

impl CometConfigModifier<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn modify_comet_config(
        bootstrap: &LegacyCosmosBootstrap,
        comet_config: &mut toml::Value,
    ) -> Result<(), Error> {
        (bootstrap.comet_config_modifier)(comet_config)
    }
}

impl DenomPrefixGetter<LegacyCosmosBootstrap, DenomForStaking> for LegacyCosmosBootstrapComponents {
    fn denom_prefix(bootstrap: &LegacyCosmosBootstrap, _label: DenomForStaking) -> &str {
        &bootstrap.staking_denom
    }
}

impl DenomPrefixGetter<LegacyCosmosBootstrap, DenomForTransfer>
    for LegacyCosmosBootstrapComponents
{
    fn denom_prefix(bootstrap: &LegacyCosmosBootstrap, _label: DenomForTransfer) -> &str {
        &bootstrap.transfer_denom
    }
}

impl AccountPrefixGetter<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn account_prefix(bootstrap: &LegacyCosmosBootstrap) -> &str {
        &bootstrap.account_prefix
    }
}

impl CompatModeGetter<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn compat_mode(bootstrap: &LegacyCosmosBootstrap) -> Option<&CompatMode> {
        bootstrap.compat_mode.as_ref()
    }
}

impl GasDenomGetter<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn gas_denom(bootstrap: &LegacyCosmosBootstrap) -> &str {
        &bootstrap.staking_denom
    }
}

impl CosmosBuilderGetter<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn cosmos_builder(bootstrap: &LegacyCosmosBootstrap) -> &CosmosBuilder {
        &bootstrap.builder
    }
}
