use cgp::prelude::*;

use crate::impls::{OsmosisQueryEip, QueryEipFromFeeMarket};
use crate::traits::{EipQuerier, EipQuerierComponent};
use crate::types::{DynamicGasConfig, EipQueryType};

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
