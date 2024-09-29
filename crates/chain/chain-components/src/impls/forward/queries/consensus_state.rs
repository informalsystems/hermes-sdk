use cgp::core::error::CanRaiseError;
use cgp::core::inner::HasInner;

use crate::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithProofs, ConsensusStateQuerier,
    ConsensusStateWithProofsQuerier,
};
use crate::traits::types::consensus_state::HasConsensusStateType;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::proof::HasCommitmentProofType;

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

impl<Chain, InChain, Counterparty, ConsensusState, CommitmentProof>
    ConsensusStateWithProofsQuerier<Chain, Counterparty> for ForwardQueryConsensusState
where
    Chain: HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>
        + HasIbcChainTypes<Counterparty>
        + HasCommitmentProofType<CommitmentProof = CommitmentProof>,
    InChain: CanQueryConsensusStateWithProofs<
        Counterparty,
        ClientId = Chain::ClientId,
        Height = Chain::Height,
        CommitmentProof = CommitmentProof,
    >,
    Counterparty: HasHeightType
        + HasConsensusStateType<Chain, ConsensusState = ConsensusState>
        + HasConsensusStateType<InChain, ConsensusState = ConsensusState>,
{
    async fn query_consensus_state_with_proofs(
        chain: &Chain,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<(ConsensusState, CommitmentProof), Chain::Error> {
        let result = chain
            .inner()
            .query_consensus_state_with_proofs(client_id, consensus_height, query_height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(result)
    }
}
