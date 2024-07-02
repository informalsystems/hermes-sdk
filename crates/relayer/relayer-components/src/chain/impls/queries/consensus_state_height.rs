use core::fmt::Debug;

use cgp_core::error::CanRaiseError;

use crate::chain::traits::queries::consensus_state_height::{
    CanQueryConsensusStateHeights, ConsensusStateHeightQuerier,
};
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct QueryConsensusStateHeightsAndFindHeightBefore;

pub struct NoConsensusStateAtLessThanHeight<'a, Chain, Counterparty>
where
    Chain: HasIbcChainTypes<Counterparty>,
    Counterparty: HasHeightType,
{
    pub chain: &'a Chain,
    pub client_id: &'a Chain::ClientId,
    pub target_height: &'a Counterparty::Height,
}

impl<Chain, Counterparty> ConsensusStateHeightQuerier<Chain, Counterparty>
    for QueryConsensusStateHeightsAndFindHeightBefore
where
    Chain: CanQueryConsensusStateHeights<Counterparty>
        + for<'a> CanRaiseError<NoConsensusStateAtLessThanHeight<'a, Chain, Counterparty>>,
    Counterparty: HasHeightType,
{
    async fn find_consensus_state_height_before(
        chain: &Chain,
        client_id: &Chain::ClientId,
        target_height: &Counterparty::Height,
    ) -> Result<Counterparty::Height, Chain::Error> {
        let heights = {
            let mut heights = chain.query_consensus_state_heights(client_id).await?;

            // sort heights in reverse order
            heights.sort_by(|a, b| b.cmp(a));
            heights
        };

        let height = heights
            .into_iter()
            .find(|height| height < target_height)
            .ok_or_else(|| {
                Chain::raise_error(NoConsensusStateAtLessThanHeight {
                    chain,
                    client_id,
                    target_height,
                })
            })?;

        Ok(height)
    }
}

impl<'a, Chain, Counterparty> Debug for NoConsensusStateAtLessThanHeight<'a, Chain, Counterparty>
where
    Chain: HasIbcChainTypes<Counterparty>,
    Counterparty: HasHeightType,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "no consensus state found that is smaller than target height {}",
            self.target_height
        )
    }
}
