use core::time::Duration;
use ibc_proto::cosmos::base::v1beta1::Coin;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

use tendermint_rpc::Url;

use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_proto::google::protobuf::Any;
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;

use crate::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use crate::types::config::gas::gas_config::GasConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CosmosChainConfig {
    pub id: String,
    pub grpc_addr: Url,
    pub account_prefix: String,
    pub key_store_folder: Option<PathBuf>,
    pub key_name: String,
    pub store_prefix: String,
    pub event_source: EventSourceMode,

    pub clock_drift: Duration,
    pub max_block_time: Duration,

    pub rpc_addr: Url,
    pub rpc_timeout: Duration,
    pub address_type: String,
    pub max_msg_num: usize,
    pub max_tx_size: usize,

    pub gas_config: GasConfig,

    pub compat_mode: Option<String>,
    pub extension_options: Vec<Any>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum EventSourceMode {
    Push {
        url: String,
    },

    #[serde(alias = "poll")]
    Pull,
}

impl From<CosmosSdkConfig> for CosmosChainConfig {
    fn from(value: CosmosSdkConfig) -> Self {
        let event_source = match value.event_source {
            ibc_relayer::config::EventSourceMode::Push {
                url,
                batch_delay: _,
            } => EventSourceMode::Push {
                url: url.to_string(),
            },
            ibc_relayer::config::EventSourceMode::Pull {
                interval: _,
                max_retries: _,
            } => EventSourceMode::Pull,
        };
        let gas_multiplier = value.gas_multiplier.unwrap_or_default().to_f64();
        let fee_granter = value.fee_granter.unwrap_or("".to_owned());
        let max_gas = value.max_gas.unwrap_or(400_000);
        let max_amount = (max_gas as f64 * gas_multiplier) * value.gas_price.price;
        let max_gas_fee_in_coin = Coin {
            denom: value.gas_price.denom.clone(),
            amount: max_amount.to_string(),
        };

        let max_fee = Fee {
            amount: vec![max_gas_fee_in_coin],
            gas_limit: max_gas,
            payer: "".to_string(),
            granter: fee_granter.clone(),
        };

        let dynamic_gas = if value.dynamic_gas_price.enabled {
            Some(DynamicGasConfig {
                multiplier: value.dynamic_gas_price.multiplier,
                max: value.dynamic_gas_price.max,
                eip_query_type: Default::default(),
                denom: value.gas_price.denom.clone(),
            })
        } else {
            None
        };
        let gas_config = GasConfig {
            default_gas: value.default_gas.unwrap_or(400_000),
            max_gas,
            gas_multiplier: value.gas_multiplier.unwrap_or_default().to_f64(),
            gas_price: value.gas_price,
            max_fee,
            fee_granter,
            dynamic_gas_config: dynamic_gas,
        };
        let mut extension_options = vec![];
        for extension_option in value.extension_options.into_iter() {
            extension_options.push(extension_option.to_any().unwrap());
        }
        Self {
            id: value.id.to_string(),
            grpc_addr: value.grpc_addr,
            account_prefix: value.account_prefix,
            key_store_folder: value.key_store_folder,
            key_name: value.key_name,
            store_prefix: value.store_prefix,
            event_source,
            rpc_addr: value.rpc_addr,
            rpc_timeout: value.rpc_timeout,
            address_type: value.address_type.to_string(),
            max_msg_num: value.max_msg_num.to_usize(),
            max_tx_size: value.max_tx_size.to_usize(),
            gas_config,
            compat_mode: value.compat_mode.map(|compat_mode| compat_mode.to_string()),
            extension_options,
            clock_drift: value.clock_drift,
            max_block_time: value.max_block_time,
        }
    }
}
