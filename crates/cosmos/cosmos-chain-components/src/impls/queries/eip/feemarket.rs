use core::str::FromStr;

use cgp::prelude::*;
use prost::DecodeError;
use subtle_encoding::base64;

use crate::impls::queries::eip::types::{EipBaseFeeHTTPResult, EipQueryError, GasPriceResponse};
use crate::traits::eip::eip_query::{EipQuerier, EipQuerierComponent};
use crate::traits::rpc_client::HasRpcClient;
use crate::types::config::gas::dynamic_gas_config::DynamicGasConfig;

/// Query EIP-1559 base fee using Skip's feemarket endpoint and decode it using
/// `GasPriceResponse`
pub struct QueryEipFromFeeMarket;

#[cgp_provider(EipQuerierComponent)]
impl<Chain> EipQuerier<Chain> for QueryEipFromFeeMarket
where
    Chain: HasRpcClient
        + CanRaiseAsyncError<reqwest::Error>
        + CanRaiseAsyncError<subtle_encoding::Error>
        + CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<core::num::ParseIntError>
        + CanRaiseAsyncError<core::num::ParseFloatError>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<EipQueryError>,
{
    async fn query_eip_base_fee(
        chain: &Chain,
        dynamic_gas_config: &DynamicGasConfig,
    ) -> Result<f64, Chain::Error> {
        let url = format!(
            "{}abci_query?path=\"/feemarket.feemarket.v1.Query/GasPrices\"&denom={}",
            chain.rpc_address(),
            dynamic_gas_config.denom,
        );

        let response = reqwest::get(&url).await.map_err(Chain::raise_error)?;

        if !response.status().is_success() {
            return Err(Chain::raise_error(EipQueryError { response }));
        }

        let result: EipBaseFeeHTTPResult = response.json().await.map_err(Chain::raise_error)?;

        let decoded = base64::decode(result.result.response.value).map_err(Chain::raise_error)?;

        let gas_price_response: GasPriceResponse =
            prost::Message::decode(decoded.as_ref()).map_err(Chain::raise_error)?;
        let dec_coin = gas_price_response
            .price
            .ok_or_else(|| Chain::raise_error("missing price in GasPriceRespone"))?;

        let raw_amount = f64::from_str(&dec_coin.amount).map_err(Chain::raise_error)?;
        let amount = raw_amount / 1000000000000000000.0;

        Ok(amount)
    }
}
