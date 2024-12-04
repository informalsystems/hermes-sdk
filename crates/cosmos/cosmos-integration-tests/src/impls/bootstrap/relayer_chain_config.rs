use core::str::FromStr;
use core::time::Duration;

use cgp::core::error::CanRaiseError;
use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::HasAccountPrefix;
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::HasDynamicGas;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;
use ibc_relayer::config::dynamic_gas::DynamicGasPrice;
use ibc_relayer::config::gas_multiplier::GasMultiplier;
use ibc_relayer::config::{self, AddressType};
use ibc_relayer::keyring::Store;
use tendermint_rpc::{Error as TendermintRpcError, Url, WebSocketClientUrl};

use crate::traits::bootstrap::compat_mode::HasCompatMode;
use crate::traits::bootstrap::relayer_chain_config::RelayerChainConfigBuilder;

pub struct BuildRelayerChainConfig;

impl<Bootstrap, Chain> RelayerChainConfigBuilder<Bootstrap> for BuildRelayerChainConfig
where
    Bootstrap: HasAccountPrefix
        + HasCompatMode
        + HasDynamicGas
        + HasChainNodeConfigType<ChainNodeConfig = CosmosChainNodeConfig>
        + HasChainGenesisConfigType<ChainGenesisConfig = CosmosGenesisConfig>
        + HasChainType<Chain = Chain>
        + CanRaiseError<TendermintRpcError>,
    Chain: HasWalletType<Wallet = CosmosTestWallet>,
{
    fn build_relayer_chain_config(
        bootstrap: &Bootstrap,
        chain_node_config: &CosmosChainNodeConfig,
        chain_genesis_config: &CosmosGenesisConfig,
        relayer_wallet: &CosmosTestWallet,
    ) -> Result<CosmosChainConfig, Bootstrap::Error> {
        let dynamic_gas_price = if let Some(dynamic_gas_config) = bootstrap.dynamic_gas() {
            DynamicGasPrice::unsafe_new(true, dynamic_gas_config.multiplier, dynamic_gas_config.max)
        } else {
            DynamicGasPrice::default()
        };
        let relayer_chain_config = CosmosSdkConfig {
            id: chain_node_config.chain_id.clone(),
            rpc_addr: Url::from_str(&format!("http://localhost:{}", chain_node_config.rpc_port))
                .map_err(Bootstrap::raise_error)?,
            grpc_addr: Url::from_str(&format!("http://localhost:{}", chain_node_config.grpc_port))
                .map_err(Bootstrap::raise_error)?,
            event_source: config::EventSourceMode::Push {
                url: WebSocketClientUrl::from_str(&format!(
                    "ws://localhost:{}/websocket",
                    chain_node_config.rpc_port
                ))
                .map_err(Bootstrap::raise_error)?,
                batch_delay: config::default::batch_delay(),
            },
            rpc_timeout: config::default::rpc_timeout(),
            trusted_node: false,
            genesis_restart: None,
            account_prefix: bootstrap.account_prefix().into(),
            key_name: relayer_wallet.id.clone(),
            key_store_type: Store::Test,
            key_store_folder: Some(chain_node_config.chain_home_dir.join("hermes_keyring")),
            store_prefix: "ibc".to_string(),
            default_gas: None,
            max_gas: Some(900000000),
            gas_adjustment: None,
            gas_multiplier: Some(GasMultiplier::unsafe_new(1.3)),
            dynamic_gas_price,
            fee_granter: None,
            max_msg_num: Default::default(),
            max_tx_size: Default::default(),
            max_grpc_decoding_size: config::default::max_grpc_decoding_size(),
            query_packets_chunk_size: config::default::query_packets_chunk_size(),
            max_block_time: Duration::from_secs(30),
            clock_drift: Duration::from_secs(5),
            trusting_period: Some(Duration::from_secs(14 * 24 * 3600)),
            client_refresh_rate: config::default::client_refresh_rate(),
            ccv_consumer_chain: false,
            trust_threshold: Default::default(),
            gas_price: config::GasPrice::new(1.0, chain_genesis_config.staking_denom.to_string()),
            packet_filter: Default::default(),
            address_type: AddressType::Cosmos,
            memo_prefix: Default::default(),
            memo_overwrite: None,
            proof_specs: Default::default(),
            extension_options: Default::default(),
            sequential_batch_tx: false,
            compat_mode: bootstrap.compat_mode().cloned(),
            clear_interval: None,
            excluded_sequences: Default::default(),
            allow_ccq: false,
        };

        Ok(relayer_chain_config.into())
    }
}
