use core::str::FromStr;

use cgp::core::error::CanRaiseError;
use ibc_proto::cosmos::base::v1beta1::DecProto;
use prost::DecodeError;
use subtle_encoding::base64;

use crate::impls::queries::eip::types::{EipBaseFeeHTTPResult, EipQueryError};
use crate::traits::eip::eip_query::EipQuerier;
use crate::traits::rpc_client::HasRpcClient;
use crate::types::config::gas::dynamic_gas_config::DynamicGasConfig;

/// Query EIP-1559 base fee using Osmosis endpoint and decode it using
/// Cosmos SDK proto `DecProto`
pub struct OsmosisQueryEip;

impl<Chain> EipQuerier<Chain> for OsmosisQueryEip
where
    Chain: HasRpcClient
        + CanRaiseError<reqwest::Error>
        + CanRaiseError<subtle_encoding::Error>
        + CanRaiseError<DecodeError>
        + CanRaiseError<core::num::ParseIntError>
        + CanRaiseError<core::num::ParseFloatError>
        + CanRaiseError<EipQueryError>,
{
    async fn query_eip_base_fee(
        chain: &Chain,
        _dynamic_gas_config: &DynamicGasConfig,
    ) -> Result<f64, Chain::Error> {
        let url = format!(
            "{}abci_query?path=\"/osmosis.txfees.v1beta1.Query/GetEipBaseFee\"",
            chain.rpc_address()
        );

        let response = reqwest::get(&url).await.map_err(Chain::raise_error)?;

        if !response.status().is_success() {
            return Err(Chain::raise_error(EipQueryError { response }));
        }

        let result: EipBaseFeeHTTPResult = response.json().await.map_err(Chain::raise_error)?;

        let decoded = base64::decode(result.result.response.value).map_err(Chain::raise_error)?;

        let dec_proto: DecProto =
            prost::Message::decode(decoded.as_ref()).map_err(Chain::raise_error)?;

        let raw_amount = f64::from_str(&dec_proto.dec).map_err(Chain::raise_error)?;
        let amount = raw_amount / 1000000000000000000.0;

        Ok(amount)
    }
}
