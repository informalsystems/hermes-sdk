use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::gas::calculate_fee;
use ibc_relayer::config::GasPrice;

use crate::impls::types::config::CosmosChainConfig;
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

impl<'a> From<&'a CosmosChainConfig> for GasConfig {
    fn from(config: &'a CosmosChainConfig) -> Self {
        let eip_query_type = if config.id.as_str().starts_with("osmo") {
            EipQueryType::Osmosis
        } else {
            EipQueryType::FeeMarket
        };
        let dynamic_gas_config = if config.dynamic_gas_price_enabled {
            Some(DynamicGasConfig {
                multiplier: config.dynamic_gas_price_multiplier,
                max: config.dynamic_gas_price_max,
                denom: config.gas_price_denom.clone(),
                eip_query_type,
            })
        } else {
            None
        };
        let gas_price = GasPrice {
            price: config.gas_price_amount,
            denom: config.gas_price_denom.clone(),
        };
        Self {
            default_gas: default_gas_from_config(config),
            max_gas: max_gas_from_config(config),
            gas_multiplier: gas_multiplier_from_config(config),
            gas_price,
            max_fee: max_fee_from_config(config),
            fee_granter: fee_granter_from_config(config),
            dynamic_gas_config,
        }
    }
}

/// Get the fee granter address
fn fee_granter_from_config(config: &CosmosChainConfig) -> String {
    config
        .fee_granter
        .clone()
        .unwrap_or("".to_owned())
        .to_string()
}

fn max_fee_from_config(config: &CosmosChainConfig) -> Fee {
    let max_gas = max_gas_from_config(config);

    let gas_price = GasPrice {
        price: config.gas_price_amount,
        denom: config.gas_price_denom.clone(),
    };

    // The maximum fee the relayer pays for a transaction
    let max_fee_in_coins = calculate_fee(max_gas, &gas_price);

    let fee_granter = fee_granter_from_config(config);

    Fee {
        amount: vec![max_fee_in_coins],
        gas_limit: max_gas,
        payer: "".to_string(),
        granter: fee_granter,
    }
}

/// The default amount of gas the relayer is willing to pay for a transaction,
/// when it cannot simulate the tx and therefore estimate the gas amount needed.
fn default_gas_from_config(config: &CosmosChainConfig) -> u64 {
    config
        .default_gas
        .unwrap_or_else(|| max_gas_from_config(config))
}

/// The maximum amount of gas the relayer is willing to pay for a transaction
fn max_gas_from_config(config: &CosmosChainConfig) -> u64 {
    config.max_gas.unwrap_or(400_000)
}

/// The gas multiplier
fn gas_multiplier_from_config(config: &CosmosChainConfig) -> f64 {
    config.gas_multiplier.unwrap_or_default()
}
