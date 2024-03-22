use alloc::sync::Arc;
use std::path::PathBuf;

use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::{DebugError, Error, ProvideCosmosError};
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk::{
    CanUseCosmosSdkChainBootstrapper, CosmosSdkBootstrapComponents, IsCosmosSdkBootstrapComponent,
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
use hermes_relayer_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
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
pub struct CosmosBootstrap {
    pub runtime: HermesRuntime,
    pub builder: Arc<CosmosBuilder>,
    pub should_randomize_identifiers: bool,
    pub chain_store_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub account_prefix: String,
    pub staking_denom: String,
    pub transfer_denom: String,
    pub genesis_config_modifier:
        Box<dyn Fn(&mut serde_json::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub comet_config_modifier:
        Box<dyn Fn(&mut toml::Value) -> Result<(), Error> + Send + Sync + 'static>,
}

impl CanUseCosmosSdkChainBootstrapper for CosmosBootstrap {}

pub struct CosmosBootstrapComponents;

impl HasComponents for CosmosBootstrap {
    type Components = CosmosBootstrapComponents;
}

delegate_all!(
    IsCosmosSdkBootstrapComponent,
    CosmosSdkBootstrapComponents,
    CosmosBootstrapComponents,
);

delegate_components! {
    CosmosBootstrapComponents {
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
            BuildCosmosChainDriver,
    }
}

impl RuntimeGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn runtime(bootstrap: &CosmosBootstrap) -> &HermesRuntime {
        &bootstrap.runtime
    }
}

impl ChainStoreDirGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn chain_store_dir(bootstrap: &CosmosBootstrap) -> &PathBuf {
        &bootstrap.chain_store_dir
    }
}

impl ChainCommandPathGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn chain_command_path(bootstrap: &CosmosBootstrap) -> &PathBuf {
        &bootstrap.chain_command_path
    }
}

impl RandomIdFlagGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn should_randomize_identifiers(bootstrap: &CosmosBootstrap) -> bool {
        bootstrap.should_randomize_identifiers
    }
}

impl CosmosGenesisConfigModifier<CosmosBootstrap> for CosmosBootstrapComponents {
    fn modify_genesis_config(
        bootstrap: &CosmosBootstrap,
        config: &mut serde_json::Value,
    ) -> Result<(), Error> {
        (bootstrap.genesis_config_modifier)(config)
    }
}

impl CometConfigModifier<CosmosBootstrap> for CosmosBootstrapComponents {
    fn modify_comet_config(
        bootstrap: &CosmosBootstrap,
        comet_config: &mut toml::Value,
    ) -> Result<(), Error> {
        (bootstrap.comet_config_modifier)(comet_config)
    }
}

impl DenomPrefixGetter<CosmosBootstrap, DenomForStaking> for CosmosBootstrapComponents {
    fn denom_prefix(bootstrap: &CosmosBootstrap, _label: DenomForStaking) -> &str {
        &bootstrap.staking_denom
    }
}

impl DenomPrefixGetter<CosmosBootstrap, DenomForTransfer> for CosmosBootstrapComponents {
    fn denom_prefix(bootstrap: &CosmosBootstrap, _label: DenomForTransfer) -> &str {
        &bootstrap.transfer_denom
    }
}

impl AccountPrefixGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn account_prefix(bootstrap: &CosmosBootstrap) -> &str {
        &bootstrap.account_prefix
    }
}

impl CompatModeGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn compat_mode(_bootstrap: &CosmosBootstrap) -> Option<&CompatMode> {
        const COMPAT_MODE: CompatMode = CompatMode::V0_37;

        Some(&COMPAT_MODE)
    }
}

impl GasDenomGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn gas_denom(bootstrap: &CosmosBootstrap) -> &str {
        &bootstrap.staking_denom
    }
}

impl CosmosBuilderGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn cosmos_builder(bootstrap: &CosmosBootstrap) -> &CosmosBuilder {
        &bootstrap.builder
    }
}
