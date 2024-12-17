use core::str::FromStr;
use core::time::Duration;

use cgp::core::error::CanRaiseError;
use hermes_cosmos_chain_components::impls::types::config::{CosmosChainConfig, EventSourceMode};
use hermes_cosmos_chain_components::types::config::gas::gas_config::{GasConfig, GasPrice};
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::HasAccountPrefix;
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::HasDynamicGas;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use tendermint_rpc::{Error as TendermintRpcError, Url};

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
        let gas_multiplier = 1.3;
        let gas_price = 1.0;
        let fee_granter = "".to_owned();
        let max_gas = 900000000;
        let max_amount = (max_gas as f64 * gas_multiplier) * gas_price;
        let max_gas_fee_in_coin = Coin {
            denom: chain_genesis_config.staking_denom.to_string(),
            amount: max_amount.to_string(),
        };

        let max_fee = Fee {
            amount: vec![max_gas_fee_in_coin],
            gas_limit: max_gas,
            payer: "".to_string(),
            granter: fee_granter.clone(),
        };

        let gas_config = GasConfig {
            default_gas: 400_000,
            max_gas,
            gas_multiplier,
            gas_price: GasPrice::new(1.0, chain_genesis_config.staking_denom.to_string()),
            max_fee,
            fee_granter,
            dynamic_gas_config: bootstrap.dynamic_gas().clone(),
        };

        let relayer_chain_config = CosmosChainConfig {
            id: chain_node_config.chain_id.to_string(),
            rpc_addr: Url::from_str(&format!("http://localhost:{}", chain_node_config.rpc_port))
                .map_err(Bootstrap::raise_error)?,
            grpc_addr: Url::from_str(&format!("http://localhost:{}", chain_node_config.grpc_port))
                .map_err(Bootstrap::raise_error)?,
            event_source: EventSourceMode::Push {
                url: format!("ws://localhost:{}/websocket", chain_node_config.rpc_port),
            },
            rpc_timeout: Duration::from_secs(10),
            account_prefix: bootstrap.account_prefix().into(),
            key_name: relayer_wallet.id.clone(),
            key_store_folder: Some(chain_node_config.chain_home_dir.join("hermes_keyring")),
            store_prefix: "ibc".to_string(),
            max_msg_num: Default::default(),
            max_tx_size: Default::default(),
            max_block_time: Duration::from_secs(30),
            clock_drift: Duration::from_secs(5),
            gas_config,
            address_type: "cosmos".to_string(),
            extension_options: Default::default(),
            compat_mode: bootstrap
                .compat_mode()
                .map(|compat_mode| compat_mode.to_string()),
        };

        Ok(relayer_chain_config)
    }
}
