use cgp::core::error::CanRaiseError;
use prost::DecodeError;

use crate::impls::queries::eip::feemarket::QueryEipFromFeeMarket;
use crate::impls::queries::eip::osmosis::OsmosisQueryEip;
use crate::impls::queries::eip::types::EipQueryError;
use crate::traits::eip::eip_query::EipQuerier;
use crate::traits::rpc_client::HasRpcClient;
use crate::types::gas::dynamic_gas_config::DynamicGasConfig;
use crate::types::gas::eip_type::EipQueryType;

pub struct DispatchQueryEip;

impl<Chain> EipQuerier<Chain> for DispatchQueryEip
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
        dynamic_gas_config: &DynamicGasConfig,
    ) -> Result<f64, Chain::Error> {
        match dynamic_gas_config.eip_query_type {
            EipQueryType::FeeMarket => {
                QueryEipFromFeeMarket::query_eip_base_fee(chain, dynamic_gas_config).await
            }
            EipQueryType::Osmosis => {
                OsmosisQueryEip::query_eip_base_fee(chain, dynamic_gas_config).await
            }
        }
    }
}
