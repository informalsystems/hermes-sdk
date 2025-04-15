use core::fmt::Display;

use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::{
    HasCommitmentProofType, HasIbcChainTypes, HasPacketCommitmentType, PacketCommitmentQuerier,
    PacketCommitmentQuerierComponent,
};
use ibc::cosmos_host::IBC_QUERY_PATH;

use crate::traits::CanQueryAbci;

pub struct QueryPacketCommitmentFromAbci;

#[cgp_provider(PacketCommitmentQuerierComponent)]
impl<Chain, Counterparty> PacketCommitmentQuerier<Chain, Counterparty>
    for QueryPacketCommitmentFromAbci
where
    Chain: HasIbcChainTypes<Counterparty>
        + HasPacketCommitmentType<Counterparty, PacketCommitment = Vec<u8>>
        + HasCommitmentProofType
        + CanQueryAbci
        + HasAsyncErrorType,
    Chain::ChannelId: Display,
{
    async fn query_packet_commitment(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequence: &Chain::Sequence,
        height: &Chain::Height,
    ) -> Result<(Option<Chain::PacketCommitment>, Chain::CommitmentProof), Chain::Error> {
        let commitment_path =
            format!("commitments/ports/{port_id}/channels/{channel_id}/sequences/{sequence}");

        let (commitment, proof) = chain
            .query_abci_with_proofs(IBC_QUERY_PATH, commitment_path.as_bytes(), height)
            .await?;

        Ok((commitment, proof))
    }
}
