use cgp::core::error::CanRaiseError;
use cgp::core::inner::HasInner;

use crate::chain::traits::queries::chain_status::{CanQueryChainStatus, ChainStatusQuerier};
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
