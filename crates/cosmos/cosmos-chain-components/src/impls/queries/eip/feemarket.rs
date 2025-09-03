use core::str::FromStr;

use hermes_prelude::*;
use prost::DecodeError;

use crate::impls::GasPriceResponse;
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
        fn encode_gas_price_request(denom: &str) -> Vec<u8> {
            // encodes feemarket's protobuf message `GasPriceRequest`.

            // Start with an empty vector to build the encoded data.
            let mut encoded_data: Vec<u8> = Vec::new();

            // 1. Add the header byte.
            // The field number is 1, and the wire type for a string is 2 (length-delimited).
            // The header is calculated as: (field_number << 3) | wire_type = (1 << 3) | 2 = 10.
            encoded_data.push(10); // 0x0A in hexadecimal

            // 2. Add the length of the string.
            // For simplicity, we assume the string length fits into a single byte.
            let len = denom.len() as u8;
            encoded_data.push(len);

            // 3. Add the raw bytes of the string.
            encoded_data.extend_from_slice(denom.as_bytes());

            encoded_data
        }

        let encoded_query = encode_gas_price_request(&dynamic_gas_config.denom);

        let abci_value = chain
            .query_abci(
                "/feemarket.feemarket.v1.Query/GasPrices",
                &encoded_query,
                None,
            )
            .await?
            .unwrap();

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
