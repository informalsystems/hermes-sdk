use core::time::Duration;
use std::path::PathBuf;

use ibc_proto::google::protobuf::Any;
use serde::{Deserialize, Serialize};
use tendermint_rpc::Url;

use crate::types::config::gas::gas_config::GasConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelayerConfig {
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub chains: Vec<CosmosChainConfig>,
}

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
