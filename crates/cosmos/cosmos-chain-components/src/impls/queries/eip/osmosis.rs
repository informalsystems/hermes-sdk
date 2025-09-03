use core::str::FromStr;

use hermes_prelude::*;
use ibc_proto::cosmos::base::v1beta1::DecProto;
use prost::DecodeError;
use subtle_encoding::base64;
use tendermint_rpc::Client;

use crate::impls::{EipBaseFeeHTTPResult, EipQueryError};
use crate::traits::{EipQuerier, EipQuerierComponent, HasRpcClient};
use crate::types::DynamicGasConfig;

/// Query EIP-1559 base fee using Osmosis endpoint and decode it using
/// Cosmos SDK proto `DecProto`
pub struct OsmosisQueryEip;

#[cgp_provider(EipQuerierComponent)]
impl<Chain> EipQuerier<Chain> for OsmosisQueryEip
where
    Chain: HasRpcClient
        + CanRaiseAsyncError<reqwest::Error>
        + CanRaiseAsyncError<subtle_encoding::Error>
        + CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<tendermint_rpc::Error>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<core::num::ParseIntError>
        + CanRaiseAsyncError<core::num::ParseFloatError>
        + CanRaiseAsyncError<EipQueryError>,
{
    async fn query_eip_base_fee(
        chain: &Chain,
        _dynamic_gas_config: &DynamicGasConfig,
    ) -> Result<f64, Chain::Error> {
        let response = chain
            .rpc_client()
            .abci_query(
                Some("/osmosis.txfees.v1beta1.Query/GetEipBaseFee".into()),
                [],
                None,
                false,
            )
            .await
            .map_err(Chain::raise_error)?;

        if !response.code.is_ok() {
            return Err(Chain::raise_error(EipQueryError { response }));
        }

        let result: EipBaseFeeHTTPResult =
            serde_json::from_slice(&response.value).map_err(Chain::raise_error)?;

        let decoded = base64::decode(result.result.response.value).map_err(Chain::raise_error)?;

        let dec_proto: DecProto =
            prost::Message::decode(decoded.as_ref()).map_err(Chain::raise_error)?;

        let raw_amount = f64::from_str(&dec_proto.dec).map_err(Chain::raise_error)?;
        let amount = raw_amount / 1000000000000000000.0;

        Ok(amount)
    }
}
