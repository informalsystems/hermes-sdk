use cgp_core::{CanRaiseError, HasInner};

use crate::chain::traits::queries::connection_end::{
    CanQueryConnectionEnd, CanQueryConnectionEndWithProofs, ConnectionEndQuerier,
    ConnectionEndWithProofsQuerier,
};
use crate::chain::traits::types::connection::HasConnectionEndType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::proof::HasCommitmentProofType;

pub struct ForwardQueryConnectionEnd;

impl<Chain, InChain, Counterparty, ConnectionEnd> ConnectionEndQuerier<Chain, Counterparty>
    for ForwardQueryConnectionEnd
where
    Chain: HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>
        + HasIbcChainTypes<Counterparty>
        + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>,
    InChain: CanQueryConnectionEnd<
            Counterparty,
            ConnectionId = Chain::ConnectionId,
            Height = Chain::Height,
        > + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>,
{
    async fn query_connection_end(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        height: &Chain::Height,
    ) -> Result<ConnectionEnd, Chain::Error> {
        let connection_end = chain
            .inner()
            .query_connection_end(connection_id, height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(connection_end)
    }
}

impl<Chain, InChain, Counterparty, ConnectionEnd, CommitmentProof>
    ConnectionEndWithProofsQuerier<Chain, Counterparty> for ForwardQueryConnectionEnd
where
    Chain: HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>
        + HasIbcChainTypes<Counterparty>
        + HasCommitmentProofType<CommitmentProof = CommitmentProof>
        + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>,
    InChain: CanQueryConnectionEndWithProofs<
            Counterparty,
            ConnectionId = Chain::ConnectionId,
            Height = Chain::Height,
        > + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasCommitmentProofType<CommitmentProof = CommitmentProof>,
{
    async fn query_connection_end_with_proofs(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        height: &Chain::Height,
    ) -> Result<(ConnectionEnd, CommitmentProof), Chain::Error> {
        let result = chain
            .inner()
            .query_connection_end_with_proofs(connection_id, height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(result)
    }
}
