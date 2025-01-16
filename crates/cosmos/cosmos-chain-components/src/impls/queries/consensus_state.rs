use cgp::core::error::CanRaiseAsyncError;
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    RawConsensusStateQuerier, RawConsensusStateWithProofsQuerier,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasRawConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ClientId;
use ibc::cosmos_host::IBC_QUERY_PATH;
use prost::{DecodeError, Message};
use prost_types::Any;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosConsensusStateFromAbci;

impl<Chain, Counterparty> RawConsensusStateQuerier<Chain, Counterparty>
    for QueryCosmosConsensusStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasRawConsensusStateType<RawConsensusState = Any>
        + CanQueryAbci
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
            .await?;

        let consensus_state_any =
            Message::decode(consensus_state_bytes.as_ref()).map_err(Chain::raise_error)?;

        Ok(consensus_state_any)
    }
}

impl<Chain, Counterparty> RawConsensusStateWithProofsQuerier<Chain, Counterparty>
    for QueryCosmosConsensusStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasRawConsensusStateType<RawConsensusState = Any>
        + HasCommitmentProofType
        + CanQueryAbci
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

        let consensus_state_any =
            Message::decode(consensus_state_bytes.as_ref()).map_err(Chain::raise_error)?;

        Ok((consensus_state_any, proofs))
    }
}
