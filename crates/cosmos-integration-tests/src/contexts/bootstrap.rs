use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::HandleErrorsWithEyre;
use cosmos_test_components::bootstrap::components::closures::cosmos_sdk_legacy::CanUseLegacyCosmosSdkChainBootstrapper;
use cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::LegacyCosmosSdkBootstrapComponents;
use cosmos_test_components::bootstrap::impls::fields::denom::DenomForStaking;
use cosmos_test_components::bootstrap::impls::fields::denom::DenomForTransfer;
use cosmos_test_components::bootstrap::impls::fields::denom::GenesisDenomGetter;
use cosmos_test_components::bootstrap::impls::generator::wallet_config::GenerateStandardWalletConfig;
use cosmos_test_components::bootstrap::impls::types::genesis_config::ProvideCosmosGenesisConfigType;
use cosmos_test_components::bootstrap::traits::chain::build_chain::ChainFromBootstrapParamsBuilder;
use cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use cosmos_test_components::bootstrap::types::chain_config::CosmosChainConfig;
use cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use cosmos_test_components::chain::types::denom::Denom;
use cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use eyre::Error;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_runtime::types::runtime::TokioRuntimeContext;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_test_components::bootstrap::traits::types::chain::ProvideChainType;
use std::path::PathBuf;
use tokio::process::Child;

use cosmos_test_components::bootstrap::impls::types::chain_config::ProvideCosmosChainConfigType;
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

use crate::contexts::chain::CosmosTestChain;

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

impl CanUseLegacyCosmosSdkChainBootstrapper for CosmosStdBootstrapContext {}

pub struct CosmosStdBootstrapComponents;

impl HasComponents for CosmosStdBootstrapContext {
    type Components = LegacyCosmosSdkBootstrapComponents<CosmosStdBootstrapComponents>;
}

delegate_components!(
    CosmosStdBootstrapComponents;
    [
        ErrorTypeComponent,
        ErrorRaiserComponent,
    ]:
        HandleErrorsWithEyre,
    ChainConfigTypeComponent: ProvideCosmosChainConfigType,
    GenesisConfigTypeComponent: ProvideCosmosGenesisConfigType,
    WalletConfigGeneratorComponent: GenerateStandardWalletConfig,
    [
        WalletConfigTypeComponent,
        WalletConfigFieldsComponent,
    ]: ProvideCosmosWalletConfigType,
);

impl ProvideChainType<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    type Chain = CosmosTestChain;
}

#[async_trait]
impl ChainFromBootstrapParamsBuilder<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    #[allow(unused_variables)]
    async fn build_chain_from_bootstrap_params(
        bootstrap: &CosmosStdBootstrapContext,
        chain_home_dir: PathBuf,
        chain_id: ChainId,
        genesis_config: CosmosGenesisConfig,
        chain_config: CosmosChainConfig,
        wallets: Vec<CosmosTestWallet>,
        chain_process: Child,
    ) -> Result<CosmosTestChain, Error> {
        Ok(CosmosTestChain)
    }
}

impl HasRuntime for CosmosStdBootstrapContext {
    type Runtime = TokioRuntimeContext;

    fn runtime(&self) -> &TokioRuntimeContext {
        &self.runtime
    }
}

impl TestDirGetter<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    fn test_dir(bootstrap: &CosmosStdBootstrapContext) -> &PathBuf {
        &bootstrap.test_dir
    }
}

impl ChainCommandPathGetter<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    fn chain_command_path(bootstrap: &CosmosStdBootstrapContext) -> &PathBuf {
        &bootstrap.chain_command_path
    }
}

impl RandomIdFlagGetter<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    fn should_randomize_identifiers(bootstrap: &CosmosStdBootstrapContext) -> bool {
        bootstrap.should_randomize_identifiers
    }
}

impl CosmosGenesisConfigModifier<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    fn modify_genesis_config(
        bootstrap: &CosmosStdBootstrapContext,
        config: &mut serde_json::Value,
    ) -> Result<(), <CosmosStdBootstrapContext as HasErrorType>::Error> {
        (bootstrap.genesis_config_modifier)(config)
    }
}

impl CometConfigModifier<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    fn modify_comet_config(
        bootstrap: &CosmosStdBootstrapContext,
        comet_config: &mut toml::Value,
    ) -> Result<(), Error> {
        (bootstrap.comet_config_modifier)(comet_config)
    }
}

impl GenesisDenomGetter<CosmosStdBootstrapContext, DenomForStaking>
    for CosmosStdBootstrapComponents
{
    fn genesis_denom(genesis_config: &CosmosGenesisConfig) -> &Denom {
        &genesis_config.staking_denom
    }
}

impl GenesisDenomGetter<CosmosStdBootstrapContext, DenomForTransfer>
    for CosmosStdBootstrapComponents
{
    fn genesis_denom(genesis_config: &CosmosGenesisConfig) -> &Denom {
        &genesis_config.transfer_denom
    }
}
