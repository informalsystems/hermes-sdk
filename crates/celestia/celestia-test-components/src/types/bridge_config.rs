pub struct CelestiaBridgeConfig {
    pub config: toml::Value,
    pub node_rpc_port: u16,
    pub node_grpc_port: u16,
    pub bridge_rpc_port: u16,
}
