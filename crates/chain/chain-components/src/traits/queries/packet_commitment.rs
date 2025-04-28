use hermes_chain_type_components::traits::{
    HasChannelIdType, HasHeightType, HasPortIdType, HasSequenceType,
};
use hermes_prelude::*;

use crate::traits::{HasCommitmentProofType, HasPacketCommitmentType};

#[cgp_component {
  provider: PacketCommitmentQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryPacketCommitment<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
    + HasPacketCommitmentType<Counterparty>
    + HasCommitmentProofType
    + HasAsyncErrorType
{
    async fn query_packet_commitment(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequence: &Self::Sequence,
        height: &Self::Height,
    ) -> Result<(Option<Self::PacketCommitment>, Self::CommitmentProof), Self::Error>;
}
