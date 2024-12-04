use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;
use ibc_relayer::chain::cosmos::gas::calculate_fee;
use ibc_relayer::chain::cosmos::types::gas::{
    default_gas_from_config, gas_multiplier_from_config, max_gas_from_config,
};
use ibc_relayer::config::GasPrice;

use crate::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use crate::types::config::gas::eip_type::EipQueryType;

pub struct GasConfig {
    pub default_gas: u64,
    pub max_gas: u64,
    pub gas_multiplier: f64,
    pub gas_price: GasPrice,
    pub max_fee: Fee,
    pub fee_granter: String,
    pub dynamic_gas_config: Option<DynamicGasConfig>,
}

impl<'a> From<&'a CosmosSdkConfig> for GasConfig {
    fn from(config: &'a CosmosSdkConfig) -> Self {
        let eip_query_type = if config.id.as_str().starts_with("osmo") {
            EipQueryType::Osmosis
        } else {
            EipQueryType::FeeMarket
        };
        let dynamic_gas_config = if config.dynamic_gas_price.enabled {
            Some(DynamicGasConfig {
                multiplier: config.dynamic_gas_price.multiplier,
                max: config.dynamic_gas_price.max,
                denom: config.gas_price.denom.clone(),
                eip_query_type,
            })
        } else {
            None
        };
        Self {
            default_gas: default_gas_from_config(config),
            max_gas: max_gas_from_config(config),
            gas_multiplier: gas_multiplier_from_config(config),
            gas_price: config.gas_price.clone(),
            max_fee: max_fee_from_config(config),
            fee_granter: fee_granter_from_config(config),
            dynamic_gas_config,
        }
    }
}

/// Get the fee granter address
fn fee_granter_from_config(config: &CosmosSdkConfig) -> String {
    config.fee_granter.as_deref().unwrap_or("").to_string()
}

fn max_fee_from_config(config: &CosmosSdkConfig) -> Fee {
    let max_gas = max_gas_from_config(config);

    // The maximum fee the relayer pays for a transaction
    let max_fee_in_coins = calculate_fee(max_gas, &config.gas_price);

    let fee_granter = fee_granter_from_config(config);

    Fee {
        amount: vec![max_fee_in_coins],
        gas_limit: max_gas,
        payer: "".to_string(),
        granter: fee_granter,
    }
}
