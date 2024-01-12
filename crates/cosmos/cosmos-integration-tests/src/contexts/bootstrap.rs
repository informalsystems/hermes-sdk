use core::str::FromStr;
use core::time::Duration;
use std::path::PathBuf;

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
use hermes_cosmos_test_components::bootstrap::impls::types::chain_config::ProvideCosmosChainConfigType;
use hermes_cosmos_test_components::bootstrap::impls::types::genesis_config::ProvideCosmosGenesisConfigType;
use hermes_cosmos_test_components::bootstrap::impls::types::wallet_config::ProvideCosmosWalletConfigType;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain::ChainFromBootstrapParamsBuilder;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::random_id::RandomIdFlagGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::test_dir::TestDirGetter;
use hermes_cosmos_test_components::bootstrap::traits::generator::generate_wallet_config::WalletConfigGeneratorComponent;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_comet_config::CometConfigModifier;
use hermes_cosmos_test_components::bootstrap::traits::modifiers::modify_genesis_config::CosmosGenesisConfigModifier;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_config::ChainConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::GenesisConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::wallet_config::{
    WalletConfigFieldsComponent, WalletConfigTypeComponent,
};
use hermes_cosmos_test_components::bootstrap::types::chain_config::CosmosChainConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain::types::denom::Denom;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_relayer_components::runtime::traits::runtime::{ProvideRuntime, RuntimeTypeComponent};
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::driver::traits::types::chain::ProvideChainType;
use hermes_test_components::driver::traits::types::chain_driver::ProvideChainDriverType;
use ibc_relayer::chain::ChainType;
use ibc_relayer::config::gas_multiplier::GasMultiplier;
use ibc_relayer::config::{self, AddressType, ChainConfig};
use ibc_relayer::keyring::Store;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use tendermint_rpc::{Url, WebSocketClientUrl};
use tokio::process::Child;

use crate::contexts::chain::CosmosChainDriver;

pub struct CosmosStdBootstrapContext {
    pub runtime: HermesRuntime,
    pub builder: CosmosBuilder,
    pub should_randomize_identifiers: bool,
    pub test_dir: PathBuf,
    pub chain_command_path: PathBuf,
    pub account_prefix: String,
    pub genesis_config_modifier:
        Box<dyn Fn(&mut serde_json::Value) -> Result<(), Error> + Send + Sync + 'static>,
    pub comet_config_modifier:
        Box<dyn Fn(&mut toml::Value) -> Result<(), Error> + Send + Sync + 'static>,
}

impl CanUseLegacyCosmosSdkChainBootstrapper for CosmosStdBootstrapContext {}

pub struct CosmosStdBootstrapComponents;

impl HasComponents for CosmosStdBootstrapContext {
    type Components = CosmosStdBootstrapComponents;
}

delegate_all!(
    IsLegacyCosmosSdkBootstrapComponent,
    LegacyCosmosSdkBootstrapComponents,
    CosmosStdBootstrapComponents,
);

delegate_components! {
    CosmosStdBootstrapComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent:
            ProvideTokioRuntimeType,
        ChainConfigTypeComponent: ProvideCosmosChainConfigType,
        GenesisConfigTypeComponent: ProvideCosmosGenesisConfigType,
        WalletConfigGeneratorComponent: GenerateStandardWalletConfig,
        [
            WalletConfigTypeComponent,
            WalletConfigFieldsComponent,
        ]: ProvideCosmosWalletConfigType,
    }
}

impl ProvideChainType<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    type Chain = CosmosChain;
}

impl ProvideChainDriverType<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    type ChainDriver = CosmosChainDriver;
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
    ) -> Result<CosmosChainDriver, Error> {
        let relayer_wallet = wallets
            .iter()
            .find(|wallet| wallet.id.starts_with("relayer"))
            .ok_or_else(|| {
                eyre!("expect relayer wallet to be provided in the list of test wallets")
            })?;

        let chain_config = ChainConfig {
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
            gas_price: config::GasPrice::new(0.003, genesis_config.staking_denom.to_string()),
            packet_filter: Default::default(),
            address_type: AddressType::Cosmos,
            memo_prefix: Default::default(),
            proof_specs: Default::default(),
            extension_options: Default::default(),
            sequential_batch_tx: false,
            compat_mode: None,
            clear_interval: None,
        };

        let base_chain = bootstrap
            .builder
            .build_chain_with_config(chain_config.clone(), Some(&relayer_wallet.keypair))
            .await?;

        let test_chain = CosmosChainDriver {
            base_chain,
            chain_config,
            full_node_process: Arc::new(chain_process),
        };

        Ok(test_chain)
    }
}

impl ProvideRuntime<CosmosStdBootstrapContext> for CosmosStdBootstrapComponents {
    fn runtime(bootstrap: &CosmosStdBootstrapContext) -> &HermesRuntime {
        &bootstrap.runtime
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
