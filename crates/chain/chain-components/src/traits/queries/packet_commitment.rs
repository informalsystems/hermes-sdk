use cgp::prelude::*;

use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::packets::receive::HasPacketCommitmentType;
use crate::traits::types::proof::HasCommitmentProofType;

#[cgp_component {
  provider: PacketCommitmentQuerier,
  context: Chain,
}]
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
