use serde_json::Value;

use crate::chain_driver::types::denom::Denom;

pub struct CosmosGenesisConfig {
    pub staking_denom: Denom,
    pub transfer_denom: Denom,
    pub config_json: Value,
}
