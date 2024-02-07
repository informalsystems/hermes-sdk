use serde::Serialize;

#[derive(Serialize)]
pub struct SovereignRollupNodeConfig {
    pub da: SovereignDaConfig,
    pub storage: SovereignStorageConfig,
    pub runner: SovereignRunnerConfig,
    pub prover_service: SovereignProverConfig,
}

#[derive(Serialize)]
pub struct SovereignDaConfig {
    pub celestia_rpc_auth_token: String,
    pub celestia_rpc_address: String,
    pub max_celestia_response_body_size: u64,
    pub celestia_rpc_timeout_seconds: u64,
}

#[derive(Serialize)]
pub struct SovereignStorageConfig {
    pub path: String,
}

#[derive(Serialize)]
pub struct SovereignRunnerConfig {
    pub start_height: u64,
    pub rpc_config: SovereignRpcConfig,
}

#[derive(Serialize)]
pub struct SovereignRpcConfig {
    pub bind_host: String,
    pub bind_port: u16,
}

#[derive(Serialize)]
pub struct SovereignProverConfig {
    pub aggregated_proof_block_jump: u64,
}
