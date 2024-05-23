use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::channel_end::{
    ChannelEndQuerier, ChannelEndWithProofsQuerier,
};
use hermes_relayer_components::chain::traits::types::channel::HasChannelEndType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc_proto::Protobuf;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::core::ics24_host::IBC_QUERY_PATH;
use ibc_relayer_types::Height;
use tendermint_proto::Error as TendermintProtoError;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosChannelEndFromAbci;

impl<Chain, Counterparty> ChannelEndQuerier<Chain, Counterparty> for QueryCosmosChannelEndFromAbci
where
    Chain: HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ChannelId = ChannelId, PortId = PortId>
        + CanQueryAbci
        + CanRaiseError<TendermintProtoError>,
{
    async fn query_channel_end(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
        height: &Height,
    ) -> Result<ChannelEnd, Chain::Error> {
        let channel_end_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let channel_end_bytes = chain
            .query_abci(IBC_QUERY_PATH, channel_end_path.as_bytes(), height)
            .await?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes).map_err(Chain::raise_error)?;

        Ok(channel_end)
    }
}

impl<Chain, Counterparty> ChannelEndWithProofsQuerier<Chain, Counterparty>
    for QueryCosmosChannelEndFromAbci
where
    Chain: HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ChannelId = ChannelId, PortId = PortId>
        + HasCommitmentProofType
        + CanQueryAbci
        + CanRaiseError<TendermintProtoError>,
{
    async fn query_channel_end_with_proofs(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
        height: &Height,
    ) -> Result<(ChannelEnd, Chain::CommitmentProof), Chain::Error> {
        let channel_end_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let (channel_end_bytes, proof) = chain
            .query_abci_with_proofs(IBC_QUERY_PATH, channel_end_path.as_bytes(), height)
            .await?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes).map_err(Chain::raise_error)?;

        Ok((channel_end, proof))
    }
}
