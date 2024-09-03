use cgp::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packets::ack::HasAcknowledgementType;
use crate::chain::traits::types::proof::HasCommitmentProofType;

#[derive_component(PacketAcknowledgementQuerierComponent, PacketAcknowledgementQuerier<Chain>)]
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
