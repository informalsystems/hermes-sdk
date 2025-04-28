use hermes_chain_type_components::traits::{
    CanUseCounterparty, HasChannelIdType, HasHeightType, HasPortIdType, HasSequenceType,
};
use hermes_prelude::*;

use crate::traits::{HasAckCommitmentHashType, HasCommitmentProofType};
use crate::types::aliases::SequenceOf;

#[cgp_component {
    provider: PacketAckCommitmentQuerier,
    context: Chain,
}]
#[async_trait]
pub trait CanQueryPacketAckCommitment<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasAckCommitmentHashType
    + HasCommitmentProofType
    + HasAsyncErrorType
    + CanUseCounterparty<Counterparty, Counterparty: HasSequenceType<Self>>
{
    async fn query_packet_ack_commitment_with_proof(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequence: &SequenceOf<Counterparty, Self>,
        height: &Self::Height,
    ) -> Result<(Self::AckCommitmentHash, Self::CommitmentProof), Self::Error>;
}
