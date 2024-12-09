use cgp::prelude::*;

use crate::traits::types::height::HasHeightType;
use crate::traits::types::status::HasChainStatusType;

/**
   Implemented by a chain context to provide method for querying the
   [current status](HasChainStatusType::ChainStatus) of the blockchain.
*/
#[cgp_component {
  provider: ChainStatusQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryChainStatus: HasChainStatusType + HasErrorType {
    /**
        Query the current status of the blockchain. The returned
        [status](HasChainStatusType::ChainStatus) is required to have the same
        or increasing
        [height](crate::traits::types::height::HasHeightType::Height)
        and
        [timestamp](crate::traits::types::timestamp::HasTimestampType::Timestamp)
        each time the query is called.

        The querying of the chain status may fail due to errors such as the full
        node not responding, or from network disconnection.

        TODO: Since the chain status can be queried frequently by the relayer,
        we should implement a cache middleware that cache the result returned
        from the chain query.
    */
    async fn query_chain_status(&self) -> Result<Self::ChainStatus, Self::Error>;
}

#[async_trait]
pub trait CanQueryChainHeight: HasHeightType + HasErrorType {
    async fn query_chain_height(&self) -> Result<Self::Height, Self::Error>;
}

impl<Chain> CanQueryChainHeight for Chain
where
    Chain: CanQueryChainStatus,
    Chain::Height: Clone,
{
    async fn query_chain_height(&self) -> Result<Chain::Height, Chain::Error> {
        let status = self.query_chain_status().await?;
        let height = Chain::chain_status_height(&status);
        Ok(height.clone())
    }
}
