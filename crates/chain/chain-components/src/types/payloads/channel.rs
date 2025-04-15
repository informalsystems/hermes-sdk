use hermes_chain_type_components::traits::HasConnectionIdType;

use crate::traits::{HasChannelEndType, HasCommitmentProofType, HasHeightType};

pub struct ChannelOpenTryPayload<Chain, Counterparty>
where
    Chain: HasChannelEndType<Counterparty> + HasHeightType + HasCommitmentProofType,
    Counterparty: HasConnectionIdType<Chain>,
{
    pub channel_end: Chain::ChannelEnd,
    pub update_height: Chain::Height,
    pub proof_init: Chain::CommitmentProof,
    pub counterparty_connection_id: Counterparty::ConnectionId,
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
