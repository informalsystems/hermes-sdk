use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerier, ConnectionEndQuerierComponent, ConnectionEndWithProofsQuerier,
    ConnectionEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::connection::HasConnectionEndType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::client::types::Height;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::ConnectionId;
use ibc::cosmos_host::IBC_QUERY_PATH;
use ibc_proto::Protobuf;
use tendermint_proto::Error as TendermintProtoError;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosConnectionEndFromAbci;

#[cgp_provider(ConnectionEndQuerierComponent)]
impl<Chain, Counterparty> ConnectionEndQuerier<Chain, Counterparty>
    for QueryCosmosConnectionEndFromAbci
where
    Chain: HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ConnectionId = ConnectionId>
        + CanQueryAbci
        + CanRaiseAsyncError<TendermintProtoError>,
{
    async fn query_connection_end(
        chain: &Chain,
        connection_id: &ConnectionId,
        height: &Height,
    ) -> Result<ConnectionEnd, Chain::Error> {
        let connection_path = format!("connections/{connection_id}");

        let connnection_end_bytes = chain
            .query_abci(IBC_QUERY_PATH, connection_path.as_bytes(), height)
            .await?;

        let connection_end =
            ConnectionEnd::decode_vec(&connnection_end_bytes).map_err(Chain::raise_error)?;

        Ok(connection_end)
    }
}

#[cgp_provider(ConnectionEndWithProofsQuerierComponent)]
impl<Chain, Counterparty> ConnectionEndWithProofsQuerier<Chain, Counterparty>
    for QueryCosmosConnectionEndFromAbci
where
    Chain: HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ConnectionId = ConnectionId>
        + HasCommitmentProofType
        + CanQueryAbci
        + CanRaiseAsyncError<TendermintProtoError>,
{
    async fn query_connection_end_with_proofs(
        chain: &Chain,
        connection_id: &ConnectionId,
        height: &Height,
    ) -> Result<(ConnectionEnd, Chain::CommitmentProof), Chain::Error> {
        let connection_path = format!("connections/{connection_id}");

        let (connnection_end_bytes, proof) = chain
            .query_abci_with_proofs(IBC_QUERY_PATH, connection_path.as_bytes(), height)
            .await?;

        let connection_end =
            ConnectionEnd::decode_vec(&connnection_end_bytes).map_err(Chain::raise_error)?;

        Ok((connection_end, proof))
    }
}
