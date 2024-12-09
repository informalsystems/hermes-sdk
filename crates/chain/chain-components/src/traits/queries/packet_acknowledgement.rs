use cgp::prelude::*;

use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::packets::ack::HasAcknowledgementType;
use crate::traits::types::proof::HasCommitmentProofType;

#[cgp_component {
  provider: PacketAcknowledgementQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryPacketAcknowledgement<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasAcknowledgementType<Counterparty>
    + HasCommitmentProofType
    + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    async fn query_packet_acknowledgement(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequence: &Counterparty::Sequence,
        height: &Self::Height,
    ) -> Result<(Self::Acknowledgement, Self::CommitmentProof), Self::Error>;
}
