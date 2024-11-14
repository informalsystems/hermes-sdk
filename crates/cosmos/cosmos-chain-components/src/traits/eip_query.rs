use cgp::prelude::*;

#[derive_component(EipQuerierComponent, EipQuerier<Chain>)]
#[async_trait]
pub trait CanQueryEipBaseFee: Async + HasErrorType {
    async fn query_eip_base_fee(&self, denom: &str) -> Result<f64, Self::Error>;
}
