use serde_json::Value;

use crate::chain::types::Denom;

#[derive(Clone)]
pub struct CosmosGenesisConfig {
    pub config_json: Value,
    pub staking_denom: Denom,
    pub transfer_denom: Denom,
}
