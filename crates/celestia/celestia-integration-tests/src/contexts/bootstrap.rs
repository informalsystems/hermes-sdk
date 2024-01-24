use std::path::PathBuf;

use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use eyre::Error;
use hermes_celestia_test_components::bootstrap::components::CelestiaBootstrapComponents as BaseCelestiaBootstrapComponents;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::BridgeBootstrapperComponent;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_celestia_test_components::bootstrap::traits::bridge_store_dir::BridgeStoreDirGetter;
use hermes_celestia_test_components::bootstrap::traits::import_bridge_key::BridgeKeyImporterComponent;
use hermes_celestia_test_components::bootstrap::traits::init_bridge_config::BridgeConfigInitializerComponent;
use hermes_celestia_test_components::bootstrap::traits::init_bridge_data::BridgeDataInitializerComponent;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_config::BridgeConfigTypeComponent;
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
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::HasChainStoreDir;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::HasRandomIdFlag;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
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

pub struct CelestiaBootstrap {
    pub cosmos_bootstrap: CosmosBootstrap,
    pub bridge_store_dir: PathBuf,
}

impl CanUseLegacyCosmosSdkChainBootstrapper for CelestiaBootstrap {}

pub struct CelestiaBootstrapComponents;

impl HasComponents for CelestiaBootstrap {
    type Components = CelestiaBootstrapComponents;
}

delegate_all!(
    IsLegacyCosmosSdkBootstrapComponent,
    LegacyCosmosSdkBootstrapComponents,
    CelestiaBootstrapComponents,
);

delegate_components! {
    CelestiaBootstrapComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RuntimeTypeComponent,
            ChainConfigTypeComponent,
            GenesisConfigTypeComponent,
            WalletConfigTypeComponent,
            WalletConfigFieldsComponent,
        ]: CosmosBootstrapComponents,
        [
            WalletConfigGeneratorComponent,
            BridgeBootstrapperComponent,
            BridgeDataInitializerComponent,
            BridgeKeyImporterComponent,
            BridgeConfigTypeComponent,
            BridgeConfigInitializerComponent,
        ]:
            BaseCelestiaBootstrapComponents,
    }
}

impl ProvideChainType<CelestiaBootstrap> for CelestiaBootstrapComponents {
    type Chain = CosmosChain;
}

impl ProvideChainDriverType<CelestiaBootstrap> for CelestiaBootstrapComponents {
    type ChainDriver = CosmosChainDriver;
}

impl ProvideRuntime<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn runtime(bootstrap: &CelestiaBootstrap) -> &HermesRuntime {
        bootstrap.cosmos_bootstrap.runtime()
    }
}

impl ChainStoreDirGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn chain_store_dir(bootstrap: &CelestiaBootstrap) -> &PathBuf {
        bootstrap.cosmos_bootstrap.chain_store_dir()
    }
}

impl BridgeStoreDirGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn bridge_store_dir(bootstrap: &CelestiaBootstrap) -> &PathBuf {
        &bootstrap.bridge_store_dir
    }
}

impl ChainCommandPathGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn chain_command_path(bootstrap: &CelestiaBootstrap) -> &PathBuf {
        bootstrap.cosmos_bootstrap.chain_command_path()
    }
}

impl RandomIdFlagGetter<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn should_randomize_identifiers(bootstrap: &CelestiaBootstrap) -> bool {
        bootstrap.cosmos_bootstrap.should_randomize_identifiers()
    }
}

impl CosmosGenesisConfigModifier<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn modify_genesis_config(
        bootstrap: &CelestiaBootstrap,
        config: &mut serde_json::Value,
    ) -> Result<(), Error> {
        bootstrap.cosmos_bootstrap.modify_genesis_config(config)
    }
}

impl CometConfigModifier<CelestiaBootstrap> for CelestiaBootstrapComponents {
    fn modify_comet_config(
        bootstrap: &CelestiaBootstrap,
        comet_config: &mut toml::Value,
    ) -> Result<(), Error> {
        bootstrap.cosmos_bootstrap.modify_comet_config(comet_config)
    }
}

impl<Label> GenesisDenomGetter<CelestiaBootstrap, Label> for CelestiaBootstrapComponents
where
    CosmosBootstrap: HasGenesisDenom<Label>,
{
    fn genesis_denom(
        bootstrap: &CelestiaBootstrap,
        label: Label,
        genesis_config: &CosmosGenesisConfig,
    ) -> Denom {
        bootstrap
            .cosmos_bootstrap
            .genesis_denom(label, genesis_config)
    }
}

#[async_trait]
impl ChainFromBootstrapParamsBuilder<CelestiaBootstrap> for CelestiaBootstrapComponents {
    async fn build_chain_from_bootstrap_params(
        bootstrap: &CelestiaBootstrap,
        chain_home_dir: PathBuf,
        chain_id: ChainId,
        genesis_config: CosmosGenesisConfig,
        chain_config: CosmosChainConfig,
        wallets: Vec<CosmosTestWallet>,
        chain_processes: Vec<Child>,
    ) -> Result<CosmosChainDriver, Error> {
        // sleep(Duration::from_secs(3)).await;
        let mut chain_driver = bootstrap
            .cosmos_bootstrap
            .build_chain_from_bootstrap_params(
                chain_home_dir,
                chain_id,
                genesis_config,
                chain_config,
                wallets,
                chain_processes,
            )
            .await?;

        let bridge_process = bootstrap.bootstrap_bridge(&chain_driver).await?;

        chain_driver.chain_processes.push(bridge_process);

        Ok(chain_driver)
    }
}
