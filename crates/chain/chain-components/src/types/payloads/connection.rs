use crate::traits::{
    HasClientStateType, HasCommitmentPrefixType, HasCommitmentProofType, HasConnectionEndType,
    HasHeightType,
};

pub struct ConnectionOpenInitPayload<Chain>
where
    Chain: HasCommitmentPrefixType,
{
    pub commitment_prefix: Chain::CommitmentPrefix,
}

pub struct ConnectionOpenTryPayload<Chain, Counterparty>
where
    Chain: HasCommitmentPrefixType
        + HasCommitmentProofType
        + HasHeightType
        + HasConnectionEndType<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasHeightType,
{
    pub commitment_prefix: Chain::CommitmentPrefix,
    pub client_state: Counterparty::ClientState,
    pub connection_end: Chain::ConnectionEnd,
    pub update_height: Chain::Height,
    pub proof_init: Chain::CommitmentProof,
    pub proof_client: Chain::CommitmentProof,
    pub proof_consensus: Chain::CommitmentProof,
    pub proof_consensus_height: Counterparty::Height,
}

pub struct ConnectionOpenAckPayload<Chain, Counterparty>
where
    Chain: HasCommitmentProofType + HasHeightType + HasConnectionEndType<Counterparty>,
    Counterparty: HasClientStateType<Chain> + HasHeightType,
{
    pub client_state: Counterparty::ClientState,
    pub connection_end: Chain::ConnectionEnd,
    pub update_height: Chain::Height,
    pub proof_try: Chain::CommitmentProof,
    pub proof_client: Chain::CommitmentProof,
    pub proof_consensus: Chain::CommitmentProof,
    pub proof_consensus_height: Counterparty::Height,
}

pub struct ConnectionOpenConfirmPayload<Chain>
where
    Chain: HasCommitmentProofType + HasHeightType,
{
    pub update_height: Chain::Height,
    pub proof_ack: Chain::CommitmentProof,
}
