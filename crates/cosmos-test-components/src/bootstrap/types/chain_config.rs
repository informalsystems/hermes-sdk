use toml::Value;

pub struct CosmosChainConfig {
    pub rpc_port: u16,
    pub p2p_port: u16,
    pub pprof_port: u16,
    pub grpc_port: u16,
    pub comet_config: Value,
    pub sdk_config: Value,
}
