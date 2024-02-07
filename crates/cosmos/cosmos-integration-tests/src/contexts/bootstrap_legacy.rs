use std::path::PathBuf;

use alloc::sync::Arc;
use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use eyre::Error;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::{
    CanUseLegacyCosmosSdkChainBootstrapper, IsLegacyCosmosSdkBootstrapComponent,
    LegacyCosmosSdkBootstrapComponents,
};
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
use hermes_relayer_components::runtime::traits::runtime::{ProvideRuntime, RuntimeTypeComponent};
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::chain_driver::traits::types::chain::ProvideChainType;
use hermes_test_components::driver::traits::types::chain_driver::ProvideChainDriverType;
use ibc_relayer::config::compat_mode::CompatMode;

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::impls::bootstrap::build_cosmos_chain::BuildCosmosChainWithNodeConfig;
use crate::impls::bootstrap::build_cosmos_chain_driver::BuildCosmosChainDriver;
use crate::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use crate::traits::bootstrap::build_chain::ChainBuilderWithNodeConfigComponent;
use crate::traits::bootstrap::compat_mode::CompatModeGetter;
use crate::traits::bootstrap::cosmos_builder::CosmosBuilderGetter;
use crate::traits::bootstrap::gas_denom::GasDenomGetter;
use crate::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilderComponent;

/**
   A bootstrap context for bootstrapping a new Cosmos chain, and builds
   a `CosmosChainDriver`.
*/
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

delegate_all!(
    IsLegacyCosmosSdkBootstrapComponent,
    LegacyCosmosSdkBootstrapComponents,
    LegacyCosmosBootstrapComponents,
);

delegate_components! {
    LegacyCosmosBootstrapComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideTokioRuntimeType,
        WalletConfigGeneratorComponent: GenerateStandardWalletConfig,
        RelayerChainConfigBuilderComponent:
            BuildRelayerChainConfig,
        ChainBuilderWithNodeConfigComponent:
            BuildCosmosChainWithNodeConfig,
        ChainDriverBuilderComponent:
            BuildCosmosChainDriver,
    }
}

impl ProvideChainType<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    type Chain = CosmosChain;
}

impl ProvideChainDriverType<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    type ChainDriver = CosmosChainDriver;
}

impl ProvideRuntime<LegacyCosmosBootstrap> for LegacyCosmosBootstrapComponents {
    fn runtime(bootstrap: &LegacyCosmosBootstrap) -> &HermesRuntime {
        &bootstrap.runtime
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
