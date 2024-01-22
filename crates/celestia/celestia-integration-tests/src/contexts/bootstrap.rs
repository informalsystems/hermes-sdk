use std::path::PathBuf;

use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use eyre::Error;
use hermes_celestia_test_components::bootstrap::impls::generator::wallet_config::GenerateCelestiaWalletConfig;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrapComponents;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::CanUseLegacyCosmosSdkChainBootstrapper;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::{
    IsLegacyCosmosSdkBootstrapComponent, LegacyCosmosSdkBootstrapComponents,
};
use hermes_cosmos_test_components::bootstrap::impls::fields::denom::GenesisDenomGetter;
use hermes_cosmos_test_components::bootstrap::impls::fields::denom::HasGenesisDenom;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain::CanBuildChainFromBootstrapParameters;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain::ChainFromBootstrapParamsBuilder;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::HasRandomIdFlag;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::test_dir::HasTestDir;
use hermes_cosmos_test_components::bootstrap::traits::fields::test_dir::TestDirGetter;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CanModifyCometConfig;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CanModifyCosmosGenesisConfig;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_config::ChainConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::GenesisConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::wallet_config::WalletConfigFieldsComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::wallet_config::WalletConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::types::chain_config::CosmosChainConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain_driver::types::denom::Denom;
use hermes_cosmos_test_components::chain_driver::types::wallet::CosmosTestWallet;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::chain_driver::traits::types::chain::ProvideChainType;
use hermes_test_components::driver::traits::types::chain_driver::ProvideChainDriverType;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use tokio::process::Child;

pub struct CelestiaAppBootstrap {
    pub cosmos_bootstrap: CosmosBootstrap,
}

impl CanUseLegacyCosmosSdkChainBootstrapper for CelestiaAppBootstrap {}

pub struct CelestiaAppBootstrapComponents;

impl HasComponents for CelestiaAppBootstrap {
    type Components = CelestiaAppBootstrapComponents;
}

delegate_all!(
    IsLegacyCosmosSdkBootstrapComponent,
    LegacyCosmosSdkBootstrapComponents,
    CelestiaAppBootstrapComponents,
);

delegate_components! {
    CelestiaAppBootstrapComponents {
        WalletConfigGeneratorComponent: GenerateCelestiaWalletConfig,
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RuntimeTypeComponent,
            ChainConfigTypeComponent,
            GenesisConfigTypeComponent,
            WalletConfigTypeComponent,
            WalletConfigFieldsComponent,
        ]: CosmosBootstrapComponents,
    }
}

impl ProvideChainType<CelestiaAppBootstrap> for CelestiaAppBootstrapComponents {
    type Chain = CosmosChain;
}

impl ProvideChainDriverType<CelestiaAppBootstrap> for CelestiaAppBootstrapComponents {
    type ChainDriver = CosmosChainDriver;
}

impl ProvideRuntime<CelestiaAppBootstrap> for CelestiaAppBootstrapComponents {
    fn runtime(bootstrap: &CelestiaAppBootstrap) -> &HermesRuntime {
        &bootstrap.cosmos_bootstrap.runtime()
    }
}

impl TestDirGetter<CelestiaAppBootstrap> for CelestiaAppBootstrapComponents {
    fn test_dir(bootstrap: &CelestiaAppBootstrap) -> &PathBuf {
        &bootstrap.cosmos_bootstrap.test_dir()
    }
}

impl ChainCommandPathGetter<CelestiaAppBootstrap> for CelestiaAppBootstrapComponents {
    fn chain_command_path(bootstrap: &CelestiaAppBootstrap) -> &PathBuf {
        &bootstrap.cosmos_bootstrap.chain_command_path()
    }
}

impl RandomIdFlagGetter<CelestiaAppBootstrap> for CelestiaAppBootstrapComponents {
    fn should_randomize_identifiers(bootstrap: &CelestiaAppBootstrap) -> bool {
        bootstrap.cosmos_bootstrap.should_randomize_identifiers()
    }
}

impl CosmosGenesisConfigModifier<CelestiaAppBootstrap> for CelestiaAppBootstrapComponents {
    fn modify_genesis_config(
        bootstrap: &CelestiaAppBootstrap,
        config: &mut serde_json::Value,
    ) -> Result<(), Error> {
        bootstrap.cosmos_bootstrap.modify_genesis_config(config)
    }
}

impl CometConfigModifier<CelestiaAppBootstrap> for CelestiaAppBootstrapComponents {
    fn modify_comet_config(
        bootstrap: &CelestiaAppBootstrap,
        comet_config: &mut toml::Value,
    ) -> Result<(), Error> {
        bootstrap.cosmos_bootstrap.modify_comet_config(comet_config)
    }
}

impl<Label> GenesisDenomGetter<CelestiaAppBootstrap, Label> for CelestiaAppBootstrapComponents
where
    CosmosBootstrap: HasGenesisDenom<Label>,
{
    fn genesis_denom(
        bootstrap: &CelestiaAppBootstrap,
        label: Label,
        genesis_config: &CosmosGenesisConfig,
    ) -> Denom {
        bootstrap
            .cosmos_bootstrap
            .genesis_denom(label, genesis_config)
    }
}

#[async_trait]
impl ChainFromBootstrapParamsBuilder<CelestiaAppBootstrap> for CelestiaAppBootstrapComponents {
    async fn build_chain_from_bootstrap_params(
        bootstrap: &CelestiaAppBootstrap,
        chain_home_dir: PathBuf,
        chain_id: ChainId,
        genesis_config: CosmosGenesisConfig,
        chain_config: CosmosChainConfig,
        wallets: Vec<CosmosTestWallet>,
        chain_processes: Vec<Child>,
    ) -> Result<CosmosChainDriver, Error> {
        bootstrap
            .cosmos_bootstrap
            .build_chain_from_bootstrap_params(
                chain_home_dir,
                chain_id,
                genesis_config,
                chain_config,
                wallets,
                chain_processes,
            )
            .await
    }
}
