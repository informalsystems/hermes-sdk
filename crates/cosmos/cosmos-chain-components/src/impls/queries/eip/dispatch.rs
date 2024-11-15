use cgp::core::error::CanRaiseError;
use prost::DecodeError;

use crate::impls::queries::eip::feemarket::QueryEipFromFeeMarket;
use crate::impls::queries::eip::osmosis::OsmosisQueryEip;
use crate::impls::queries::eip::types::EipQueryError;
use crate::traits::eip::eip_query::EipQuerier;
use crate::traits::eip::eip_type::{EipQueryType, HasEipQueryType};
use crate::traits::rpc_client::HasRpcClient;

pub struct DispatchQueryEip;

impl<Chain> EipQuerier<Chain> for DispatchQueryEip
where
    Chain: HasRpcClient
        + HasEipQueryType
        + CanRaiseError<reqwest::Error>
        + CanRaiseError<subtle_encoding::Error>
        + CanRaiseError<DecodeError>
        + CanRaiseError<core::num::ParseIntError>
        + CanRaiseError<core::num::ParseFloatError>
        + CanRaiseError<EipQueryError>,
{
    async fn query_eip_base_fee(chain: &Chain, denom: &str) -> Result<f64, Chain::Error> {
        match chain.eip_query_type() {
            EipQueryType::FeeMarket => {
                QueryEipFromFeeMarket::query_eip_base_fee(chain, denom).await
            }
            EipQueryType::Osmosis => OsmosisQueryEip::query_eip_base_fee(chain, denom).await,
        }
    }
}
