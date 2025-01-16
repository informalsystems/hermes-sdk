use cgp::prelude::*;
use hermes_chain_type_components::traits::types::counterparty::CanUseCounterparty;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;

use crate::traits::types::packets::ack::HasAcknowledgementType;
use crate::traits::types::proof::HasCommitmentProofType;
use crate::types::aliases::SequenceOf;

#[cgp_component {
  provider: PacketAcknowledgementQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryPacketAcknowledgement<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasAcknowledgementType<Counterparty>
    + HasCommitmentProofType
    + HasAsyncErrorType
    + CanUseCounterparty<Counterparty, Counterparty: HasSequenceType<Self>>
{
    async fn query_packet_acknowledgement(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequence: &SequenceOf<Counterparty, Self>,
        height: &Self::Height,
    ) -> Result<(Self::Acknowledgement, Self::CommitmentProof), Self::Error>;
}
