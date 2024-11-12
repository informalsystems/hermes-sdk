use core::cmp::min;

use cgp::prelude::CanRaiseError;
use hermes_chain_type_components::traits::fields::chain_id::HasChainId;
use hermes_relayer_components::transaction::traits::types::fee::HasFeeType;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::eip_base_fee::query_eip_base_fee;
use ibc_relayer::chain::cosmos::gas::{mul_ceil, mul_floor};
use ibc_relayer::config::GasPrice;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::traits::convert_gas_to_fee::GasToFeeConverter;
use crate::traits::gas_config::HasGasConfig;
use crate::traits::rpc_client::HasRpcClient;

pub struct StaticConvertCosmosGasToFee;

impl<Chain> GasToFeeConverter<Chain> for StaticConvertCosmosGasToFee
where
    Chain: HasFeeType<Fee = Fee>
        + HasChainId<ChainId = ChainId>
        + HasRpcClient
        + HasGasConfig
        + CanRaiseError<&'static str>,
{
    async fn gas_amount_to_fee(chain: &Chain, gas_used: u64) -> Result<Chain::Fee, Chain::Error> {
        let adjusted_gas_limit = adjust_estimated_gas(
            chain.gas_config().gas_multiplier,
            chain.gas_config().max_gas,
            gas_used,
        );

        // The fee in coins based on gas amount
        let fee_amount = mul_ceil(adjusted_gas_limit, chain.gas_config().gas_price.price);

        let amount = Coin {
            denom: chain.gas_config().gas_price.denom.to_string(),
            amount: fee_amount.to_string(),
        };

        Ok(Fee {
            amount: vec![amount],
            gas_limit: adjusted_gas_limit,
            payer: "".to_string(),
            granter: chain.gas_config().fee_granter.clone(),
        })
    }
}

pub struct DynamicConvertCosmosGasToFee;

impl<Chain> GasToFeeConverter<Chain> for DynamicConvertCosmosGasToFee
where
    Chain: HasFeeType<Fee = Fee>
        + HasChainId<ChainId = ChainId>
        + HasRpcClient
        + HasGasConfig
        + CanRaiseError<&'static str>,
{
    async fn gas_amount_to_fee(chain: &Chain, gas_used: u64) -> Result<Chain::Fee, Chain::Error> {
        let adjusted_gas_limit = adjust_estimated_gas(
            chain.gas_config().gas_multiplier,
            chain.gas_config().max_gas,
            gas_used,
        );

        let dynamic_gas_price = query_eip_base_fee(
            chain.rpc_address(),
            &chain.gas_config().gas_price.denom,
            chain.chain_id(),
        )
        .await
        .map(|base_fee| base_fee * chain.gas_config().dynamic_gas_price.multiplier)
        .map(|new_price| GasPrice {
            price: new_price,
            denom: chain.gas_config().gas_price.denom.clone(),
        });

        let mut dynamic_gas_price =
            dynamic_gas_price.unwrap_or(chain.gas_config().gas_price.clone());

        dynamic_gas_price = if dynamic_gas_price.price > chain.gas_config().dynamic_gas_price.max {
            GasPrice::new(
                chain.gas_config().dynamic_gas_price.max,
                dynamic_gas_price.denom,
            )
        } else {
            dynamic_gas_price
        };

        // The fee in coins based on gas amount
        let fee_amount = mul_ceil(adjusted_gas_limit, dynamic_gas_price.price);

        let amount = Coin {
            denom: chain.gas_config().gas_price.denom.to_string(),
            amount: fee_amount.to_string(),
        };

        Ok(Fee {
            amount: vec![amount],
            gas_limit: adjusted_gas_limit,
            payer: "".to_string(),
            granter: chain.gas_config().fee_granter.clone(),
        })
    }
}

/// Adjusts the fee based on the configured `gas_multiplier` to prevent out of gas errors.
/// The actual gas cost, when a transaction is executed, may be slightly higher than the
/// one returned by the simulation.
fn adjust_estimated_gas(gas_multiplier: f64, max_gas: u64, gas_amount: u64) -> u64 {
    // No need to compute anything if the gas amount is zero
    if gas_amount == 0 {
        return 0;
    };

    // If the multiplier is 1, no need to perform the multiplication
    if gas_multiplier == 1.0 {
        return min(gas_amount, max_gas);
    }

    // Multiply the gas estimate by the gas_multiplier option
    let (_sign, digits) = mul_floor(gas_amount, gas_multiplier).to_u64_digits();

    let gas = match digits.as_slice() {
        // If there are no digits it means that the resulting amount is zero.
        [] => 0,

        // If there is a single "digit", it means that the result fits in a u64, so we can use that.
        [gas] => *gas,

        // Otherwise, the multiplication overflow and we use u64::MAX instead.
        _ => u64::MAX,
    };

    // Bound the gas estimate by the max_gas option
    min(gas, max_gas)
}
