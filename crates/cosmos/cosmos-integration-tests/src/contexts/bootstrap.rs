use core::str::FromStr;
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
use ibc_relayer::chain::ChainType;
use ibc_relayer::config::compat_mode::CompatMode;
use ibc_relayer::config::gas_multiplier::GasMultiplier;
use ibc_relayer::config::{self, AddressType, ChainConfig};
use ibc_relayer::keyring::Store;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use tendermint_rpc::{Url, WebSocketClientUrl};
use tokio::process::Child;
use tokio::time::sleep;

use crate::contexts::chain_driver::CosmosChainDriver;

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
        chain_home_dir: PathBuf,
        chain_id: ChainId,
        genesis_config: CosmosGenesisConfig,
        chain_config: CosmosChainNodeConfig,
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

        let relayer_chain_config = ChainConfig {
            id: chain_id.clone(),
            r#type: ChainType::CosmosSdk,
            rpc_addr: Url::from_str(&format!("http://localhost:{}", chain_config.rpc_port))?,
            grpc_addr: Url::from_str(&format!("http://localhost:{}", chain_config.grpc_port))?,
            event_source: config::EventSourceMode::Push {
                url: WebSocketClientUrl::from_str(&format!(
                    "ws://localhost:{}/websocket",
                    chain_config.rpc_port
                ))?,
                batch_delay: config::default::batch_delay(),
            },
            rpc_timeout: config::default::rpc_timeout(),
            trusted_node: false,
            genesis_restart: None,
            account_prefix: bootstrap.account_prefix.clone(),
            key_name: relayer_wallet.id.clone(),
            key_store_type: Store::Test,
            key_store_folder: Some(chain_home_dir.join("hermes_keyring")),
            store_prefix: "ibc".to_string(),
            default_gas: None,
            max_gas: Some(3000000),
            gas_adjustment: None,
            gas_multiplier: Some(GasMultiplier::unsafe_new(1.2)),
            fee_granter: None,
            max_msg_num: Default::default(),
            max_tx_size: Default::default(),
            max_grpc_decoding_size: config::default::max_grpc_decoding_size(),
            max_block_time: Duration::from_secs(30),
            clock_drift: Duration::from_secs(5),
            trusting_period: Some(Duration::from_secs(14 * 24 * 3600)),
            ccv_consumer_chain: false,
            trust_threshold: Default::default(),
            gas_price: config::GasPrice::new(0.003, bootstrap.staking_denom.to_string()),
            packet_filter: Default::default(),
            address_type: AddressType::Cosmos,
            memo_prefix: Default::default(),
            proof_specs: Default::default(),
            extension_options: Default::default(),
            sequential_batch_tx: false,
            compat_mode: bootstrap.compat_mode.clone(),
            clear_interval: None,
        };

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
            chain_home_dir,
            chain_config,
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
