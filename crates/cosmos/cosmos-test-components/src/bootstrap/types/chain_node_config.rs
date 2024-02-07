use std::path::PathBuf;

use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use toml::Value;

#[derive(Clone)]
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
