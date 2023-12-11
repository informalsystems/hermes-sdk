use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use eyre::Error;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::error::TokioRuntimeError;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_test_components::bootstrap::traits::types::chain::ProvideChainType;
use std::io::Error as IoError;
use std::path::PathBuf;

use crate::bootstrap::components::cosmos_sdk::CosmosSdkBootstrapComponents;
use crate::bootstrap::impls::types::chain_config::ProvideCosmosChainConfigType;
use crate::bootstrap::impls::types::genesis_config::ProvideJsonGenesisConfigType;
use crate::bootstrap::impls::types::wallet_config::ProvideCosmosWalletConfigType;
use crate::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use crate::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use crate::bootstrap::traits::fields::test_dir::TestDirGetter;
use crate::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;
use crate::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use crate::bootstrap::traits::types::chain_config::ChainConfigTypeComponent;
use crate::bootstrap::traits::types::genesis_config::GenesisConfigTypeComponent;
use crate::bootstrap::traits::types::wallet_config::WalletConfigFieldsComponent;
use crate::bootstrap::traits::types::wallet_config::WalletConfigTypeComponent;

pub struct CosmosBootstrapContext {
    pub runtime: TokioRuntimeContext,
    pub should_randomize_identifiers: bool,
    pub test_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub genesis_config_modifier:
        Box<dyn Fn(&mut serde_json::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub comet_config_modifier:
        Box<dyn Fn(&mut toml::Value) -> Result<(), Error> + Send + Sync + 'static>,
}

pub struct CosmosBootstrapComponents;

impl HasComponents for CosmosBootstrapContext {
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

impl ProvideChainType<CosmosBootstrapContext> for CosmosBootstrapComponents {
    // TODO
    type Chain = ();
}

impl HasErrorType for CosmosBootstrapContext {
    type Error = Error;
}

impl CanRaiseError<IoError> for CosmosBootstrapContext {
    fn raise_error(e: IoError) -> Error {
        e.into()
    }
}

impl HasRuntime for CosmosBootstrapContext {
    type Runtime = TokioRuntimeContext;

    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }

    fn runtime_error(e: TokioRuntimeError) -> Error {
        e.into()
    }
}

impl TestDirGetter<CosmosBootstrapContext> for CosmosBootstrapComponents {
    fn test_dir(bootstrap: &CosmosBootstrapContext) -> &PathBuf {
        &bootstrap.test_dir
    }
}

impl ChainCommandPathGetter<CosmosBootstrapContext> for CosmosBootstrapComponents {
    fn chain_command_path(bootstrap: &CosmosBootstrapContext) -> &PathBuf {
        &bootstrap.chain_command_path
    }
}

impl RandomIdFlagGetter<CosmosBootstrapContext> for CosmosBootstrapComponents {
    fn should_randomize_identifiers(bootstrap: &CosmosBootstrapContext) -> bool {
        bootstrap.should_randomize_identifiers
    }
}

impl CosmosGenesisConfigModifier<CosmosBootstrapContext> for CosmosBootstrapComponents {
    fn modify_genesis_config(
        bootstrap: &CosmosBootstrapContext,
        config: &mut serde_json::Value,
    ) -> Result<(), <CosmosBootstrapContext as HasErrorType>::Error> {
        (bootstrap.genesis_config_modifier)(config)
    }
}

impl CometConfigModifier<CosmosBootstrapContext> for CosmosBootstrapComponents {
    fn modify_comet_config(
        bootstrap: &CosmosBootstrapContext,
        comet_config: &mut toml::Value,
    ) -> Result<(), Error> {
        (bootstrap.comet_config_modifier)(comet_config)
    }
}
