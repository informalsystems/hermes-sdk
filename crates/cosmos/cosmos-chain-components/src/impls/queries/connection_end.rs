use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerier, ConnectionEndWithProofsQuerier,
};
use hermes_relayer_components::chain::traits::types::connection::HasConnectionEndType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::connection::types::ConnectionEnd;
use ibc_proto::Protobuf;
use ibc_relayer_types::core::ics24_host::identifier::ConnectionId;
use ibc_relayer_types::core::ics24_host::IBC_QUERY_PATH;
use ibc_relayer_types::Height;
use tendermint_proto::Error as TendermintProtoError;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosConnectionEndFromAbci;

impl<Chain, Counterparty> ConnectionEndQuerier<Chain, Counterparty>
    for QueryCosmosConnectionEndFromAbci
where
    Chain: HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ConnectionId = ConnectionId>
        + CanQueryAbci
        + CanRaiseError<TendermintProtoError>,
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

impl<Chain, Counterparty> ConnectionEndWithProofsQuerier<Chain, Counterparty>
    for QueryCosmosConnectionEndFromAbci
where
    Chain: HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ConnectionId = ConnectionId>
        + HasCommitmentProofType
        + CanQueryAbci
        + CanRaiseError<TendermintProtoError>,
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
