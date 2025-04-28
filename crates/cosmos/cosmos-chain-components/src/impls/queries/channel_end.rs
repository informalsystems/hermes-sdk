use hermes_core::relayer_components::chain::traits::{
    ChannelEndQuerier, ChannelEndQuerierComponent, ChannelEndWithProofsQuerier,
    ChannelEndWithProofsQuerierComponent, HasChannelEndType, HasCommitmentProofType,
    HasIbcChainTypes,
};
use hermes_prelude::*;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, PortId};
use ibc::cosmos_host::IBC_QUERY_PATH;
use ibc_proto::Protobuf;
use tendermint_proto::Error as TendermintProtoError;

use crate::traits::CanQueryAbci;

pub struct QueryCosmosChannelEndFromAbci;

#[cgp_provider(ChannelEndQuerierComponent)]
impl<Chain, Counterparty> ChannelEndQuerier<Chain, Counterparty> for QueryCosmosChannelEndFromAbci
where
    Chain: HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ChannelId = ChannelId, PortId = PortId>
        + CanQueryAbci
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<TendermintProtoError>,
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
            .await?
            .ok_or_else(|| {
                Chain::raise_error(format!("channel not found: {channel_id}/{port_id}"))
            })?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes).map_err(Chain::raise_error)?;

        Ok(channel_end)
    }
}

#[cgp_provider(ChannelEndWithProofsQuerierComponent)]
impl<Chain, Counterparty> ChannelEndWithProofsQuerier<Chain, Counterparty>
    for QueryCosmosChannelEndFromAbci
where
    Chain: HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ChannelId = ChannelId, PortId = PortId>
        + HasCommitmentProofType
        + CanQueryAbci
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<TendermintProtoError>,
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

        let channel_end_bytes = channel_end_bytes.ok_or_else(|| {
            Chain::raise_error(format!("channel not found: {channel_id}/{port_id}"))
        })?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes).map_err(Chain::raise_error)?;

        Ok((channel_end, proof))
    }
}
