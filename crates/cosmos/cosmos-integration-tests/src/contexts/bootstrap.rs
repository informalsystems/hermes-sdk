use core::time::Duration;
use std::path::PathBuf;

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use eyre::{eyre, Error};
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk_legacy::{
    CanUseLegacyCosmosSdkChainBootstrapper, IsLegacyCosmosSdkBootstrapComponent,
    LegacyCosmosSdkBootstrapComponents,
};
use hermes_cosmos_test_components::bootstrap::impls::fields::denom::{
    DenomForStaking, DenomForTransfer, GenesisDenomGetter,
};
use hermes_cosmos_test_components::bootstrap::impls::generator::wallet_config::GenerateStandardWalletConfig;
use hermes_cosmos_test_components::bootstrap::impls::types::chain_node_config::ProvideCosmosChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::impls::types::genesis_config::ProvideCosmosGenesisConfigType;
use hermes_cosmos_test_components::bootstrap::impls::types::wallet_config::ProvideCosmosWalletConfigType;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain::ChainFromBootstrapParamsBuilder;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::AccountPrefixGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::ChainNodeConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::GenesisConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::wallet_config::{
    WalletConfigFieldsComponent, WalletConfigTypeComponent,
};
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain_driver::types::denom::Denom;
use hermes_cosmos_test_components::chain_driver::types::wallet::CosmosTestWallet;
use hermes_relayer_components::chain::traits::components::chain_status_querier::CanQueryChainStatus;
use hermes_relayer_components::runtime::traits::runtime::{ProvideRuntime, RuntimeTypeComponent};
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::chain_driver::traits::types::chain::ProvideChainType;
use hermes_test_components::driver::traits::types::chain_driver::ProvideChainDriverType;
use ibc_relayer::config::compat_mode::CompatMode;
use tokio::process::Child;
use tokio::time::sleep;

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::impls::bootstrap::relayer_chain_config::BuildRelayerChainConfig;
use crate::traits::bootstrap::compat_mode::CompatModeGetter;
use crate::traits::bootstrap::gas_denom::GasDenomGetter;
use crate::traits::bootstrap::relayer_chain_config::{
    CanBuildRelayerChainConfig, RelayerChainConfigBuilderComponent,
};

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
    pub staking_denom: Denom,
    pub transfer_denom: Denom,
    pub compat_mode: Option<CompatMode>,
    pub genesis_config_modifier:
        Box<dyn Fn(&mut serde_json::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub comet_config_modifier:
        Box<dyn Fn(&mut toml::Value) -> Result<(), Error> + Send + Sync + 'static>,
}

impl CanUseLegacyCosmosSdkChainBootstrapper for CosmosBootstrap {}

pub struct CosmosBootstrapComponents;

impl HasComponents for CosmosBootstrap {
    type Components = CosmosBootstrapComponents;
}

delegate_all!(
    IsLegacyCosmosSdkBootstrapComponent,
    LegacyCosmosSdkBootstrapComponents,
    CosmosBootstrapComponents,
);

delegate_components! {
    CosmosBootstrapComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideTokioRuntimeType,
        ChainNodeConfigTypeComponent: ProvideCosmosChainNodeConfigType,
        GenesisConfigTypeComponent: ProvideCosmosGenesisConfigType,
        WalletConfigGeneratorComponent: GenerateStandardWalletConfig,
        [
            WalletConfigTypeComponent,
            WalletConfigFieldsComponent,
        ]: ProvideCosmosWalletConfigType,
        RelayerChainConfigBuilderComponent:
            BuildRelayerChainConfig,
    }
}

impl ProvideChainType<CosmosBootstrap> for CosmosBootstrapComponents {
    type Chain = CosmosChain;
}

impl ProvideChainDriverType<CosmosBootstrap> for CosmosBootstrapComponents {
    type ChainDriver = CosmosChainDriver;
}

#[async_trait]
impl ChainFromBootstrapParamsBuilder<CosmosBootstrap> for CosmosBootstrapComponents {
    async fn build_chain_from_bootstrap_params(
        bootstrap: &CosmosBootstrap,
        genesis_config: CosmosGenesisConfig,
        chain_node_config: CosmosChainNodeConfig,
        wallets: BTreeMap<String, CosmosTestWallet>,
        chain_processes: Vec<Child>,
    ) -> Result<CosmosChainDriver, Error> {
        let relayer_wallet = wallets
            .get("relayer")
            .ok_or_else(|| {
                eyre!("expect relayer wallet to be provided in the list of test wallets")
            })?
            .clone();

        let user_wallet_a = wallets
            .get("user1")
            .ok_or_else(|| eyre!("expect user1 wallet to be provided in the list of test wallets"))?
            .clone();

        let user_wallet_b = wallets
            .get("user2")
            .ok_or_else(|| eyre!("expect user2 wallet to be provided in the list of test wallets"))?
            .clone();

        let relayer_chain_config =
            bootstrap.build_relayer_chain_config(&chain_node_config, &relayer_wallet)?;

        let base_chain = bootstrap
            .builder
            .build_chain_with_config(
                relayer_chain_config.clone(),
                Some(&relayer_wallet.keypair.clone()),
            )
            .await?;

        for _ in 0..10 {
            sleep(Duration::from_secs(1)).await;

            // Wait for full node process to start up. We do this by waiting
            // the chain to reach at least height 2 after starting.
            if let Ok(status) = base_chain.query_chain_status().await {
                if status.height.revision_height() > 1 {
                    break;
                }
            }
        }

        let test_chain = CosmosChainDriver {
            base_chain,
            chain_node_config,
            genesis_config,
            relayer_chain_config,
            chain_processes,
            staking_denom: bootstrap.staking_denom.clone(),
            transfer_denom: bootstrap.transfer_denom.clone(),
            relayer_wallet: relayer_wallet.clone(),
            user_wallet_a: user_wallet_a.clone(),
            user_wallet_b: user_wallet_b.clone(),
            wallets,
        };

        Ok(test_chain)
    }
}

impl ProvideRuntime<CosmosBootstrap> for CosmosBootstrapComponents {
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

impl GenesisDenomGetter<CosmosBootstrap, DenomForStaking> for CosmosBootstrapComponents {
    fn genesis_denom(
        bootstrap: &CosmosBootstrap,
        _label: DenomForStaking,
        _genesis_config: &CosmosGenesisConfig,
    ) -> Denom {
        bootstrap.staking_denom.clone()
    }
}

impl GenesisDenomGetter<CosmosBootstrap, DenomForTransfer> for CosmosBootstrapComponents {
    fn genesis_denom(
        bootstrap: &CosmosBootstrap,
        _label: DenomForTransfer,
        _genesis_config: &CosmosGenesisConfig,
    ) -> Denom {
        bootstrap.transfer_denom.clone()
    }
}

impl AccountPrefixGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn account_prefix(bootstrap: &CosmosBootstrap) -> &str {
        &bootstrap.account_prefix
    }
}

impl CompatModeGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn compat_mode(bootstrap: &CosmosBootstrap) -> Option<&CompatMode> {
        bootstrap.compat_mode.as_ref()
    }
}

impl GasDenomGetter<CosmosBootstrap> for CosmosBootstrapComponents {
    fn gas_denom(bootstrap: &CosmosBootstrap) -> &Denom {
        &bootstrap.staking_denom
    }
}
