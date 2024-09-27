use crate::traits::types::height::HasHeightType;
use crate::traits::types::packets::ack::HasAcknowledgementType;
use crate::traits::types::proof::HasCommitmentProofType;

pub struct ReceivePacketPayload<Chain>
where
    Chain: HasHeightType + HasCommitmentProofType,
{
    pub update_height: Chain::Height,
    pub proof_commitment: Chain::CommitmentProof,
}

pub struct AckPacketPayload<Chain, Counterparty>
where
    Chain: HasHeightType + HasCommitmentProofType + HasAcknowledgementType<Counterparty>,
{
    pub ack: Chain::Acknowledgement,
    pub update_height: Chain::Height,
    pub proof_ack: Chain::CommitmentProof,
}

pub struct TimeoutUnorderedPacketPayload<Chain>
where
    Chain: HasHeightType + HasCommitmentProofType,
{
    pub update_height: Chain::Height,
    pub proof_unreceived: Chain::CommitmentProof,
}
