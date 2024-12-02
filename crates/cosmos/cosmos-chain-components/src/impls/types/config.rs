use core::time::Duration;
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CosmosChainConfig {
    pub id: String,
    pub grpc_addr: String,
    pub account_prefix: String,
    pub key_store_folder: Option<PathBuf>,
    pub key_name: String,
    pub store_prefix: String,
    pub event_source: EventSourceMode,

    pub clock_drift: Duration,
    pub max_block_time: Duration,

    pub rpc_addr: String,
    pub rpc_timeout: Duration,
    pub address_type: String,
    pub max_msg_num: usize,
    pub max_tx_size: usize,

    pub default_gas: Option<u64>,
    pub max_gas: Option<u64>,
    pub gas_multiplier: Option<f64>,
    pub gas_price_amount: f64,
    pub gas_price_denom: String,
    pub fee_granter: Option<String>,
    pub dynamic_gas_price_enabled: bool,
    pub dynamic_gas_price_multiplier: f64,
    pub dynamic_gas_price_max: f64,

    pub compat_mode: Option<String>,
    pub extension_options: Vec<String>,
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
        Self {
            id: value.id.to_string(),
            grpc_addr: value.grpc_addr.to_string(),
            account_prefix: value.account_prefix,
            key_store_folder: value.key_store_folder,
            key_name: value.key_name,
            store_prefix: value.store_prefix,
            event_source,
            rpc_addr: value.rpc_addr.to_string(),
            rpc_timeout: value.rpc_timeout,
            address_type: value.address_type.to_string(),
            max_msg_num: value.max_msg_num.to_usize(),
            max_tx_size: value.max_tx_size.to_usize(),
            default_gas: value.default_gas,
            max_gas: value.max_gas,
            gas_multiplier: value
                .gas_multiplier
                .map(|gas_multiplier| gas_multiplier.to_f64()),
            gas_price_amount: value.gas_price.price,
            gas_price_denom: value.gas_price.denom,
            fee_granter: value.fee_granter,
            dynamic_gas_price_enabled: value.dynamic_gas_price.enabled,
            dynamic_gas_price_multiplier: value.dynamic_gas_price.multiplier,
            dynamic_gas_price_max: value.dynamic_gas_price.max,
            compat_mode: value.compat_mode.map(|compat_mode| compat_mode.to_string()),
            extension_options: value
                .extension_options
                .iter()
                .map(|extension_option| extension_option.to_string())
                .collect(),
            clock_drift: value.clock_drift,
            max_block_time: value.max_block_time,
        }
    }
}
