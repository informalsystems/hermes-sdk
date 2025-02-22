use cgp::prelude::*;

use crate::impls::queries::eip::feemarket::QueryEipFromFeeMarket;
use crate::impls::queries::eip::osmosis::OsmosisQueryEip;
use crate::traits::eip::eip_query::{EipQuerier, EipQuerierComponent};
use crate::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use crate::types::config::gas::eip_type::EipQueryType;

pub struct DispatchQueryEip;

#[cgp_provider(EipQuerierComponent)]
impl<Chain> EipQuerier<Chain> for DispatchQueryEip
where
    QueryEipFromFeeMarket: EipQuerier<Chain>,
    OsmosisQueryEip: EipQuerier<Chain>,
    Chain: HasAsyncErrorType + Async,
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
