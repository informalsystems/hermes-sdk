use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use eyre::Error;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::error::TokioRuntimeError;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_test_components::bootstrap::traits::types::chain::ProvideChainType;
use std::io::Error as IoError;
use std::path::PathBuf;

// use cosmos_test_components::bootstrap::components::closures::cosmos_sdk::CanUseCosmosSdkChainBootstrapper;
use cosmos_test_components::bootstrap::components::cosmos_sdk::CosmosSdkBootstrapComponents;
use cosmos_test_components::bootstrap::impls::types::chain_config::ProvideCosmosChainConfigType;
use cosmos_test_components::bootstrap::impls::types::genesis_config::ProvideJsonGenesisConfigType;
use cosmos_test_components::bootstrap::impls::types::wallet_config::ProvideCosmosWalletConfigType;
use cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use cosmos_test_components::bootstrap::traits::fields::test_dir::TestDirGetter;
use cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;
use cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use cosmos_test_components::bootstrap::traits::types::chain_config::ChainConfigTypeComponent;
use cosmos_test_components::bootstrap::traits::types::genesis_config::GenesisConfigTypeComponent;
use cosmos_test_components::bootstrap::traits::types::wallet_config::WalletConfigFieldsComponent;
use cosmos_test_components::bootstrap::traits::types::wallet_config::WalletConfigTypeComponent;

use crate::contexts::chain::CosmosChain;

pub struct CosmosStdBootstrapContext {
    pub runtime: TokioRuntimeContext,
    pub should_randomize_identifiers: bool,
    pub test_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub genesis_config_modifier:
        Box<dyn Fn(&mut serde_json::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub comet_config_modifier:
        Box<dyn Fn(&mut toml::Value) -> Result<(), Error> + Send + Sync + 'static>,
}

// impl CanUseCosmosSdkChainBootstrapper for CosmosStdBootstrapContext {}

pub struct CosmosBootstrapComponents;

impl HasComponents for CosmosStdBootstrapContext {
    type Components = CosmosSdkBootstrapComponents<CosmosBootstrapComponents>;
}

delegate_components!(
    CosmosBootstrapComponents;
    ChainConfigTypeComponent: ProvideCosmosChainConfigType,
    GenesisConfigTypeComponent: ProvideJsonGenesisConfigType,
    [
        WalletConfigTypeComponent,
        WalletConfigFieldsComponent,
    ]: ProvideCosmosWalletConfigType,
);

impl ProvideChainType<CosmosStdBootstrapContext> for CosmosBootstrapComponents {
    type Chain = CosmosChain<BaseChainHandle>;
}

impl HasErrorType for CosmosStdBootstrapContext {
    type Error = Error;
}

impl CanRaiseError<IoError> for CosmosStdBootstrapContext {
    fn raise_error(e: IoError) -> Error {
        e.into()
    }
}

impl HasRuntime for CosmosStdBootstrapContext {
    type Runtime = TokioRuntimeContext;

    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }

    fn runtime_error(e: TokioRuntimeError) -> Error {
        e.into()
    }
}

impl TestDirGetter<CosmosStdBootstrapContext> for CosmosBootstrapComponents {
    fn test_dir(bootstrap: &CosmosStdBootstrapContext) -> &PathBuf {
        &bootstrap.test_dir
    }
}

impl ChainCommandPathGetter<CosmosStdBootstrapContext> for CosmosBootstrapComponents {
    fn chain_command_path(bootstrap: &CosmosStdBootstrapContext) -> &PathBuf {
        &bootstrap.chain_command_path
    }
}

impl RandomIdFlagGetter<CosmosStdBootstrapContext> for CosmosBootstrapComponents {
    fn should_randomize_identifiers(bootstrap: &CosmosStdBootstrapContext) -> bool {
        bootstrap.should_randomize_identifiers
    }
}

impl CosmosGenesisConfigModifier<CosmosStdBootstrapContext> for CosmosBootstrapComponents {
    fn modify_genesis_config(
        bootstrap: &CosmosStdBootstrapContext,
        config: &mut serde_json::Value,
    ) -> Result<(), <CosmosStdBootstrapContext as HasErrorType>::Error> {
        (bootstrap.genesis_config_modifier)(config)
    }
}

impl CometConfigModifier<CosmosStdBootstrapContext> for CosmosBootstrapComponents {
    fn modify_comet_config(
        bootstrap: &CosmosStdBootstrapContext,
        comet_config: &mut toml::Value,
    ) -> Result<(), Error> {
        (bootstrap.comet_config_modifier)(comet_config)
    }
}
