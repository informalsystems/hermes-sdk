use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::{
    HasCommitmentProofType, HasHeightFields, HasIbcChainTypes, HasRawConsensusStateType,
    RawConsensusStateQuerier, RawConsensusStateQuerierComponent,
    RawConsensusStateWithProofsQuerier, RawConsensusStateWithProofsQuerierComponent,
};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ClientId;
use ibc::cosmos_host::IBC_QUERY_PATH;
use prost::{DecodeError, Message};
use prost_types::Any;

use crate::traits::CanQueryAbci;

pub struct QueryCosmosConsensusStateFromAbci;

#[cgp_provider(RawConsensusStateQuerierComponent)]
impl<Chain, Counterparty> RawConsensusStateQuerier<Chain, Counterparty>
    for QueryCosmosConsensusStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasRawConsensusStateType<RawConsensusState = Any>
        + CanQueryAbci
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<DecodeError>,
    Counterparty: HasHeightFields,
{
    async fn query_raw_consensus_state(
        chain: &Chain,
        client_id: &ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Height,
    ) -> Result<Any, Chain::Error> {
        let revision_number = Counterparty::revision_number(consensus_height);
        let revision_height = Counterparty::revision_height(consensus_height);

        let consensus_state_path =
            format!("clients/{client_id}/consensusStates/{revision_number}-{revision_height}");

        let consensus_state_bytes = chain
            .query_abci(
                IBC_QUERY_PATH,
                consensus_state_path.as_bytes(),
                query_height,
            )
            .await?
            .ok_or_else(|| Chain::raise_error(format!("consensus state not found: {client_id}")))?;

        let consensus_state_any =
            Message::decode(consensus_state_bytes.as_ref()).map_err(Chain::raise_error)?;

        Ok(consensus_state_any)
    }
}

#[cgp_provider(RawConsensusStateWithProofsQuerierComponent)]
impl<Chain, Counterparty> RawConsensusStateWithProofsQuerier<Chain, Counterparty>
    for QueryCosmosConsensusStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasRawConsensusStateType<RawConsensusState = Any>
        + HasCommitmentProofType
        + CanQueryAbci
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<DecodeError>,
    Counterparty: HasHeightFields,
{
    async fn query_raw_consensus_state_with_proofs(
        chain: &Chain,
        client_id: &ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Height,
    ) -> Result<(Any, Chain::CommitmentProof), Chain::Error> {
        let revision_number = Counterparty::revision_number(consensus_height);
        let revision_height = Counterparty::revision_height(consensus_height);

        let consensus_state_path =
            format!("clients/{client_id}/consensusStates/{revision_number}-{revision_height}");

        let (consensus_state_bytes, proofs) = chain
            .query_abci_with_proofs(
                IBC_QUERY_PATH,
                consensus_state_path.as_bytes(),
                query_height,
            )
            .await?;

        let consensus_state_bytes = consensus_state_bytes
            .ok_or_else(|| Chain::raise_error(format!("consensus state not found: {client_id}")))?;

        let consensus_state_any =
            Message::decode(consensus_state_bytes.as_ref()).map_err(Chain::raise_error)?;

        Ok((consensus_state_any, proofs))
    }
}
