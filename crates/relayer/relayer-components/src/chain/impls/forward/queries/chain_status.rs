use cgp_core::{CanRaiseError, HasInner};

use crate::chain::traits::queries::chain_status::{
    CanQueryChainStatus, CanQueryChainStatusAtHeight, ChainStatusAtHeightQuerier,
    ChainStatusQuerier,
};
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::status::HasChainStatusType;

pub struct ForwardQueryChainStatus;

impl<Chain, InChain> ChainStatusQuerier<Chain> for ForwardQueryChainStatus
where
    Chain: HasInner<Inner = InChain> + HasChainStatusType + CanRaiseError<InChain::Error>,
    InChain: CanQueryChainStatus<ChainStatus = Chain::ChainStatus>,
{
    async fn query_chain_status(chain: &Chain) -> Result<Chain::ChainStatus, Chain::Error> {
        let chain_status = chain
            .inner()
            .query_chain_status()
            .await
            .map_err(Chain::raise_error)?;

        Ok(chain_status)
    }
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
