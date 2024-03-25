use cgp_core::prelude::HasErrorType;
use cgp_core::DelegateComponent;
use core::marker::PhantomData;

use crate::chain::traits::queries::consensus_state::ConsensusStateQuerier;
use crate::chain::traits::types::consensus_state::HasConsensusStateType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct DelegateQueryConsensusState<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> ConsensusStateQuerier<Chain, Counterparty>
    for DelegateQueryConsensusState<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasConsensusStateType<Chain> + HasHeightType,
    Delegate: ConsensusStateQuerier<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_consensus_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        Delegate::query_consensus_state(chain, client_id, consensus_height, query_height).await
    }
}
