use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;

#[derive_component(AbciQuerierComponent, AbciQuerier<Chain>)]
#[async_trait]
pub trait CanQueryAbci: HasHeightType + HasErrorType {
    async fn query_abci(
        &self,
        path: &str,
        data: &[u8],
        height: &Self::Height,
    ) -> Result<Vec<u8>, Self::Error>;
}
