use serde_json::Value;

use crate::chain_driver::types::denom::Denom;

#[derive(Clone)]
pub struct CosmosGenesisConfig {
    pub config_json: Value,
    pub staking_denom: Denom,
    pub transfer_denom: Denom,
}
