use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packets::receive::HasPacketCommitmentType;
use crate::chain::traits::types::proof::HasCommitmentProofType;

#[derive_component(PacketCommitmentQuerierComponent, PacketCommitmentQuerier<Chain>)]
#[async_trait]
pub trait CanQueryPacketCommitment<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasPacketCommitmentType<Counterparty>
    + HasCommitmentProofType
    + HasErrorType
{
    async fn query_packet_commitment(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequence: &Self::Sequence,
        height: &Self::Height,
    ) -> Result<(Self::PacketCommitment, Self::CommitmentProof), Self::Error>;
}
