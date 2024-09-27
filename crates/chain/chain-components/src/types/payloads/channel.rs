use crate::traits::types::channel::HasChannelEndType;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::proof::HasCommitmentProofType;

pub struct ChannelOpenTryPayload<Chain, Counterparty>
where
    Chain: HasChannelEndType<Counterparty> + HasHeightType + HasCommitmentProofType,
{
    pub channel_end: Chain::ChannelEnd,
    pub update_height: Chain::Height,
    pub proof_init: Chain::CommitmentProof,
}

pub struct ChannelOpenAckPayload<Chain, Counterparty>
where
    Chain: HasChannelEndType<Counterparty> + HasHeightType + HasCommitmentProofType,
{
    pub channel_end: Chain::ChannelEnd,
    pub update_height: Chain::Height,
    pub proof_try: Chain::CommitmentProof,
}

pub struct ChannelOpenConfirmPayload<Chain>
where
    Chain: HasHeightType + HasCommitmentProofType,
{
    pub update_height: Chain::Height,
    pub proof_ack: Chain::CommitmentProof,
}
