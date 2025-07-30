use std::path::PathBuf;

use ibc::core::host::types::identifiers::ChainId;
use toml::Value;

#[derive(Clone, Debug)]
pub struct CosmosChainNodeConfig {
    pub chain_id: ChainId,
    pub chain_home_dir: PathBuf,
    pub rpc_port: u16,
    pub p2p_port: u16,
    pub pprof_port: u16,
    pub grpc_port: u16,
    pub comet_config: Value,
    pub sdk_config: Value,
}
