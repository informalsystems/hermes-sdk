use core::str::FromStr;

use hermes_prelude::*;
use prost::DecodeError;

use crate::impls::{GasPriceRequest, GasPriceResponse};
use crate::traits::{CanQueryAbci, EipQuerier, EipQuerierComponent};
use crate::types::DynamicGasConfig;

/// Query EIP-1559 base fee using Skip's feemarket endpoint and decode it using
/// `GasPriceResponse`
pub struct QueryEipFromFeeMarket;

#[cgp_provider(EipQuerierComponent)]
impl<Chain> EipQuerier<Chain> for QueryEipFromFeeMarket
where
    Chain: CanQueryAbci
        + CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<core::num::ParseFloatError>
        + CanRaiseAsyncError<&'static str>,
{
    async fn query_eip_base_fee(
        chain: &Chain,
        dynamic_gas_config: &DynamicGasConfig,
    ) -> Result<f64, Chain::Error> {
        let gas_price_request = GasPriceRequest {
            denom: Some(dynamic_gas_config.denom.clone()),
        };

        let proto_encoded = prost::Message::encode_to_vec(&gas_price_request);

        let abci_value = chain
            .query_abci(
                "/feemarket.feemarket.v1.Query/GasPrice",
                &proto_encoded,
                None,
            )
            .await?
            .ok_or_else(|| Chain::raise_error("GasPrice response is empty"))?;

        let gas_price_response: GasPriceResponse =
            prost::Message::decode(abci_value.as_ref()).map_err(Chain::raise_error)?;
        let dec_coin = gas_price_response
            .price
            .ok_or_else(|| Chain::raise_error("missing price in GasPriceResponse"))?;

        let raw_amount = f64::from_str(&dec_coin.amount).map_err(Chain::raise_error)?;
        let amount = raw_amount / 1000000000000000000.0;

        Ok(amount)
    }
}
