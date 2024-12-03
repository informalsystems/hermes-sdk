use serde::Deserialize;
use serde::Serialize;

use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::config::GasPrice;

use crate::types::config::gas::dynamic_gas_config::DynamicGasConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GasConfig {
    pub default_gas: u64,
    pub max_gas: u64,
    pub gas_multiplier: f64,
    pub gas_price: GasPrice,
    pub max_fee: Fee,
    pub fee_granter: String,
    pub dynamic_gas_config: Option<DynamicGasConfig>,
}
