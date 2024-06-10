use cgp_core::prelude::*;

use cgp_core::CanRaiseError;
use cgp_core::HasInner;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;

#[derive_component(ChainStatusAtHeightQuerierComponent, ChainStatusAtHeightQuerier<Chain>)]
#[async_trait]
pub trait CanQueryChainStatusAtHeight: HasHeightType + HasChainStatusType + HasErrorType {
    async fn query_chain_status_at_height(
        &self,
        height: &Self::Height,
    ) -> Result<Self::ChainStatus, Self::Error>;
}

pub struct ForwardQueryChainStatusAtHeight;

impl<Chain, InChain> ChainStatusAtHeightQuerier<Chain> for ForwardQueryChainStatusAtHeight
where
    Chain: HasInner<Inner = InChain> + HasChainStatusType + CanRaiseError<InChain::Error>,
    InChain: CanQueryChainStatusAtHeight<ChainStatus = Chain::ChainStatus>,
    InChain: HasHeightType<Height = Chain::Height>,
{
    async fn query_chain_status_at_height(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Chain::ChainStatus, Chain::Error> {
        let chain_status = chain
            .inner()
            .query_chain_status_at_height(height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(chain_status)
    }
}
