use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::types::AsBytes;

use crate::traits::queries::consensus_state::{
    CanQueryRawConsensusState, CanQueryRawConsensusStateWithProofs, ConsensusStateQuerier,
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerier,
    ConsensusStateWithProofsQuerierComponent,
};
use crate::traits::types::consensus_state::HasConsensusStateType;
use crate::traits::types::height::HasHeightType;

pub struct QueryAndConvertRawConsensusState;

#[cgp_provider(ConsensusStateQuerierComponent)]
impl<Chain, Counterparty, Encoding> ConsensusStateQuerier<Chain, Counterparty>
    for QueryAndConvertRawConsensusState
where
    Chain: CanQueryRawConsensusState<Counterparty> + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasConsensusStateType<Chain>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>
        + HasHeightType,
    Encoding: Async + CanConvert<Chain::RawConsensusState, Counterparty::ConsensusState>,
{
    async fn query_consensus_state(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        let raw_consensus_state = chain
            .query_raw_consensus_state(client_id, consensus_height, query_height)
            .await?;

        let consensus_state = Counterparty::default_encoding()
            .convert(&raw_consensus_state)
            .map_err(Chain::raise_error)?;

        Ok(consensus_state)
    }
}

#[cgp_provider(ConsensusStateWithProofsQuerierComponent)]
impl<Chain, Counterparty, Encoding> ConsensusStateWithProofsQuerier<Chain, Counterparty>
    for QueryAndConvertRawConsensusState
where
    Chain: CanQueryRawConsensusStateWithProofs<Counterparty> + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasConsensusStateType<Chain>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>
        + HasHeightType,
    Encoding: Async + CanConvert<Chain::RawConsensusState, Counterparty::ConsensusState>,
{
    async fn query_consensus_state_with_proofs(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<(Counterparty::ConsensusState, Chain::CommitmentProof), Chain::Error> {
        let (raw_consensus_state, proofs) = chain
            .query_raw_consensus_state_with_proofs(client_id, consensus_height, query_height)
            .await?;

        let consensus_state = Counterparty::default_encoding()
            .convert(&raw_consensus_state)
            .map_err(Chain::raise_error)?;

        Ok((consensus_state, proofs))
    }
}
