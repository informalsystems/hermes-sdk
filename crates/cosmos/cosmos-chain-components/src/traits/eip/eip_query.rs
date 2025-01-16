use cgp::prelude::*;

use crate::types::config::gas::dynamic_gas_config::DynamicGasConfig;

#[cgp_component {
  provider: EipQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryEipBaseFee: Async + HasAsyncErrorType {
    async fn query_eip_base_fee(
        &self,
        dynamic_gas_config: &DynamicGasConfig,
    ) -> Result<f64, Self::Error>;
}
