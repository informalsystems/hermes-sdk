use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::{DelegateComponent, HasErrorType};

use crate::traits::queries::consensus_state_height::ConsensusStateHeightsQuerier;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::ibc::HasIbcChainTypes;

pub struct DelegateQueryConsensusStateHeights<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> ConsensusStateHeightsQuerier<Chain, Counterparty>
    for DelegateQueryConsensusStateHeights<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasHeightType,
    Delegate: ConsensusStateHeightsQuerier<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_consensus_state_heights(
        chain: &Chain,
        client_id: &Chain::ClientId,
    ) -> Result<Vec<Counterparty::Height>, Chain::Error> {
        Delegate::query_consensus_state_heights(chain, client_id).await
    }
}
