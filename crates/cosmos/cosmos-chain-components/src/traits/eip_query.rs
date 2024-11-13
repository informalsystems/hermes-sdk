use cgp::prelude::*;

#[derive_component(EipQuerierComponent, EipQuerier<Chain>)]
#[async_trait]
pub trait CanQueryEipBaseFee: HasErrorType + Send + Sync + 'static {
    async fn query_eip_base_fee(&self, path: &str) -> Result<f64, Self::Error>;
}
