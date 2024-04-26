use cgp_core::{CanRaiseError, HasInner};

use crate::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, ConsensusStateQuerier,
};
use crate::chain::traits::types::consensus_state::HasConsensusStateType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct ForwardQueryConsensusState;

impl<Chain, InChain, Counterparty, ConsensusState> ConsensusStateQuerier<Chain, Counterparty>
    for ForwardQueryConsensusState
where
    Chain:
        HasInner<Inner = InChain> + CanRaiseError<InChain::Error> + HasIbcChainTypes<Counterparty>,
    InChain:
        CanQueryConsensusState<Counterparty, ClientId = Chain::ClientId, Height = Chain::Height>,
    Counterparty: HasHeightType
        + HasConsensusStateType<Chain, ConsensusState = ConsensusState>
        + HasConsensusStateType<InChain, ConsensusState = ConsensusState>,
{
    async fn query_consensus_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<ConsensusState, Chain::Error> {
        let consensus_state = chain
            .inner()
            .query_consensus_state(client_id, consensus_height, query_height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(consensus_state)
    }
}
