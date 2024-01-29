use cgp_core::prelude::*;
use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use ibc_relayer::chain::endpoint::ChainStatus;
use ibc_relayer::chain::handle::ChainHandle;

use crate::traits::chain_handle::HasBlockingChainHandle;

pub struct QueryChainStatusWithChainHandle;

#[async_trait]
impl<Chain> ChainStatusQuerier<Chain> for QueryChainStatusWithChainHandle
where
    Chain: HasErrorType + HasChainStatusType<ChainStatus = ChainStatus> + HasBlockingChainHandle,
{
    async fn query_chain_status(chain: &Chain) -> Result<Chain::ChainStatus, Chain::Error> {
        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let status = chain_handle
                    .query_application_status()
                    .map_err(Chain::raise_error)?;

                Ok(status)
            })
            .await
    }
}
