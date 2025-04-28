use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{CanUseCounterparty, ConsensusStateOf, HasClientIdType};
use hermes_prelude::*;

use super::chain_status::CanQueryChainStatus;
use crate::traits::{
    HasCommitmentProofType, HasConsensusStateType, HasHeightType, HasRawConsensusStateType,
};
use crate::types::aliases::HeightOf;

#[cgp_component {
  provider: ConsensusStateQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryConsensusState<Counterparty>:
    HasClientIdType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasConsensusStateType<Self> + HasHeightType>
    + HasHeightType
    + HasAsyncErrorType
{
    async fn query_consensus_state(
        &self,
        tag: PhantomData<Counterparty>,
        client_id: &Self::ClientId,
        consensus_height: &HeightOf<Counterparty>,
        query_height: &Self::Height,
    ) -> Result<ConsensusStateOf<Counterparty, Self>, Self::Error>;
}

#[cgp_component {
  provider: ConsensusStateWithProofsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryConsensusStateWithProofs<Counterparty>:
    HasClientIdType<Counterparty>
    + HasHeightType
    + HasCommitmentProofType
    + HasAsyncErrorType
    + CanUseCounterparty<Counterparty, Counterparty: HasConsensusStateType<Self> + HasHeightType>
{
    async fn query_consensus_state_with_proofs(
        &self,
        tag: PhantomData<Counterparty>,
        client_id: &Self::ClientId,
        consensus_height: &HeightOf<Counterparty>,
        query_height: &Self::Height,
    ) -> Result<(ConsensusStateOf<Counterparty, Self>, Self::CommitmentProof), Self::Error>;
}

#[cgp_component {
  provider: RawConsensusStateQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryRawConsensusState<Counterparty>:
    HasClientIdType<Counterparty> + HasHeightType + HasRawConsensusStateType + HasAsyncErrorType
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

#[cgp_component {
  provider: RawConsensusStateWithProofsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryRawConsensusStateWithProofs<Counterparty>:
    HasClientIdType<Counterparty>
    + HasHeightType
    + HasRawConsensusStateType
    + HasCommitmentProofType
    + HasAsyncErrorType
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
    HasClientIdType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasConsensusStateType<Self> + HasHeightType>
    + HasAsyncErrorType
{
    async fn query_consensus_state_with_latest_height(
        &self,
        tag: PhantomData<Counterparty>,
        client_id: &Self::ClientId,
        consensus_height: &HeightOf<Counterparty>,
    ) -> Result<ConsensusStateOf<Counterparty, Self>, Self::Error>;
}

impl<Chain, Counterparty> CanQueryConsensusStateWithLatestHeight<Counterparty> for Chain
where
    Chain: CanQueryConsensusState<Counterparty> + CanQueryChainStatus,
    Counterparty: HasConsensusStateType<Chain> + HasHeightType,
{
    async fn query_consensus_state_with_latest_height(
        &self,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        let status = self.query_chain_status().await?;

        self.query_consensus_state(
            tag,
            client_id,
            consensus_height,
            Chain::chain_status_height(&status),
        )
        .await
    }
}

#[cgp_provider(ConsensusStateQuerierComponent)]
impl<Chain, Counterparty, Components, Delegate> ConsensusStateQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasClientIdType<Counterparty> + HasHeightType + HasAsyncErrorType,
    Counterparty: HasConsensusStateType<Chain> + HasHeightType,
    Delegate: ConsensusStateQuerier<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_consensus_state(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        Delegate::query_consensus_state(chain, tag, client_id, consensus_height, query_height).await
    }
}

#[cgp_provider(ConsensusStateWithProofsQuerierComponent)]
impl<Chain, Counterparty, Components, Delegate> ConsensusStateWithProofsQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain:
        HasClientIdType<Counterparty> + HasHeightType + HasCommitmentProofType + HasAsyncErrorType,
    Counterparty: HasConsensusStateType<Chain> + HasHeightType,
    Delegate: ConsensusStateWithProofsQuerier<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_consensus_state_with_proofs(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<(Counterparty::ConsensusState, Chain::CommitmentProof), Chain::Error> {
        Delegate::query_consensus_state_with_proofs(
            chain,
            tag,
            client_id,
            consensus_height,
            query_height,
        )
        .await
    }
}
