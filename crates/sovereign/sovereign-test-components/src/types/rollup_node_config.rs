use serde::Serialize;

#[derive(Serialize)]
pub struct SovereignRollupNodeConfig {
    pub da: SovereignDaConfig,
    pub storage: SovereignStorageConfig,
    pub runner: SovereignRunnerConfig,
    pub proof_manager: SovereignProverConfig,
}

#[derive(Serialize)]
pub struct SovereignDaConfig {
    pub celestia_rpc_auth_token: String,
    pub celestia_rpc_address: String,
    pub max_celestia_response_body_size: u64,
    pub celestia_rpc_timeout_seconds: u64,
    pub own_celestia_address: String,
}

#[derive(Serialize)]
pub struct SovereignStorageConfig {
    pub path: String,
}

#[derive(Serialize)]
pub struct SovereignRunnerConfig {
    pub genesis_height: u64,
    pub rpc_config: SovereignRpcConfig,
    pub axum_config: SovereignAxumConfig,
    pub da_polling_interval_ms: u64,
}

#[derive(Serialize)]
pub struct SovereignRpcConfig {
    pub bind_host: String,
    pub bind_port: u16,
}

#[derive(Serialize)]
pub struct SovereignAxumConfig {
    pub bind_host: String,
    pub bind_port: u16,
}

#[derive(Serialize)]
pub struct SovereignProverConfig {
    pub aggregated_proof_block_jump: u64,
}
