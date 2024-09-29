use cgp::prelude::*;

use super::chain_status::CanQueryChainStatus;
use crate::traits::types::consensus_state::{HasConsensusStateType, HasRawConsensusStateType};
use crate::traits::types::height::HasHeightType;
use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::proof::HasCommitmentProofType;

#[derive_component(ConsensusStateQuerierComponent, ConsensusStateQuerier<Chain>)]
#[async_trait]
pub trait CanQueryConsensusState<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasConsensusStateType<Self> + HasHeightType,
{
    async fn query_consensus_state(
        &self,
        client_id: &Self::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Self::Height,
    ) -> Result<Counterparty::ConsensusState, Self::Error>;
}

#[derive_component(ConsensusStateWithProofsQuerierComponent, ConsensusStateWithProofsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryConsensusStateWithProofs<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasCommitmentProofType + HasErrorType
where
    Counterparty: HasConsensusStateType<Self> + HasHeightType,
{
    async fn query_consensus_state_with_proofs(
        &self,
        client_id: &Self::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Self::Height,
    ) -> Result<(Counterparty::ConsensusState, Self::CommitmentProof), Self::Error>;
}

#[derive_component(RawConsensusStateQuerierComponent, RawConsensusStateQuerier<Chain>)]
#[async_trait]
pub trait CanQueryRawConsensusState<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasRawConsensusStateType + HasErrorType
where
    Counterparty: HasHeightType,
{
    async fn query_raw_consensus_state(
        &self,
        client_id: &Self::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Self::Height,
    ) -> Result<Self::RawConsensusState, Self::Error>;
}

#[derive_component(RawConsensusStateWithProofsQuerierComponent, RawConsensusStateWithProofsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryRawConsensusStateWithProofs<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasRawConsensusStateType + HasCommitmentProofType + HasErrorType
where
    Counterparty: HasHeightType,
{
    async fn query_raw_consensus_state_with_proofs(
        &self,
        client_id: &Self::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Self::Height,
    ) -> Result<(Self::RawConsensusState, Self::CommitmentProof), Self::Error>;
}

#[async_trait]
pub trait CanQueryConsensusStateWithLatestHeight<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasConsensusStateType<Self> + HasHeightType,
{
    async fn query_consensus_state_with_latest_height(
        &self,
        client_id: &Self::ClientId,
        consensus_height: &Counterparty::Height,
    ) -> Result<Counterparty::ConsensusState, Self::Error>;
}

impl<Chain, Counterparty> CanQueryConsensusStateWithLatestHeight<Counterparty> for Chain
where
    Chain: CanQueryConsensusState<Counterparty> + CanQueryChainStatus,
    Counterparty: HasConsensusStateType<Chain> + HasHeightType,
{
    async fn query_consensus_state_with_latest_height(
        &self,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        let status = self.query_chain_status().await?;

        self.query_consensus_state(
            client_id,
            consensus_height,
            Chain::chain_status_height(&status),
        )
        .await
    }
}
