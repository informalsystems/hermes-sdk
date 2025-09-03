use core::str::FromStr;

use hermes_prelude::*;
use ibc_proto::cosmos::base::v1beta1::DecProto;
use prost::DecodeError;

use crate::traits::{CanQueryAbci, EipQuerier, EipQuerierComponent};
use crate::types::DynamicGasConfig;

/// Query EIP-1559 base fee using Osmosis endpoint and decode it using
/// Cosmos SDK proto `DecProto`
pub struct OsmosisQueryEip;

#[cgp_provider(EipQuerierComponent)]
impl<Chain> EipQuerier<Chain> for OsmosisQueryEip
where
    Chain: CanQueryAbci
        + CanRaiseAsyncError<subtle_encoding::Error>
        + CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<core::num::ParseIntError>
        + CanRaiseAsyncError<core::num::ParseFloatError>,
{
    async fn query_eip_base_fee(
        chain: &Chain,
        _dynamic_gas_config: &DynamicGasConfig,
    ) -> Result<f64, Chain::Error> {
        let abci_value = chain
            .query_abci("/osmosis.txfees.v1beta1.Query/GetEipBaseFee", &[], None)
            .await?
            .unwrap();

        let dec_proto: DecProto =
            prost::Message::decode(abci_value.as_ref()).map_err(Chain::raise_error)?;

        let raw_amount = f64::from_str(&dec_proto.dec).map_err(Chain::raise_error)?;
        let amount = raw_amount / 1000000000000000000.0;

        Ok(amount)
    }
}
