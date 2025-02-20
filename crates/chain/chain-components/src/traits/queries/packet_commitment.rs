use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;

use crate::traits::types::packets::receive::HasPacketCommitmentType;
use crate::traits::types::proof::HasCommitmentProofType;

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
