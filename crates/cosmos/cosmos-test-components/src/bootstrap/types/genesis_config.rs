use serde_json::Value;

#[derive(Clone)]
pub struct CosmosGenesisConfig {
    pub config_json: Value,
}
