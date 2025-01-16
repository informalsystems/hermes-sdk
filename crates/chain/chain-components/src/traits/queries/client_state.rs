use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::counterparty::CanUseCounterparty;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::ibc::client_state::ClientStateOf;

use crate::traits::queries::chain_status::CanQueryChainStatus;
use crate::traits::types::client_state::{HasClientStateType, HasRawClientStateType};
use crate::traits::types::proof::HasCommitmentProofType;

#[cgp_component {
  provider: ClientStateQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryClientState<Counterparty>:
    HasClientIdType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasClientStateType<Self>>
    + HasHeightType
    + HasAsyncErrorType
{
    async fn query_client_state(
        &self,
        tag: PhantomData<Counterparty>,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<ClientStateOf<Counterparty, Self>, Self::Error>;
}

#[cgp_component {
  provider: ClientStateWithProofsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryClientStateWithProofs<Counterparty>:
    HasHeightType + HasClientIdType<Counterparty> + HasCommitmentProofType + HasAsyncErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_client_state_with_proofs(
        &self,
        tag: PhantomData<Counterparty>,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<(Counterparty::ClientState, Self::CommitmentProof), Self::Error>;
}

#[cgp_component {
  provider: RawClientStateQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryRawClientState<Counterparty>:
    HasClientIdType<Counterparty> + HasHeightType + HasRawClientStateType + HasAsyncErrorType
{
    async fn query_raw_client_state(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Self::RawClientState, Self::Error>;
}

#[cgp_component {
  provider: RawClientStateWithProofsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryRawClientStateWithProofs<Counterparty>:
    HasClientIdType<Counterparty>
    + HasHeightType
    + HasRawClientStateType
    + HasCommitmentProofType
    + HasAsyncErrorType
{
    async fn query_raw_client_state_with_proofs(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<(Self::RawClientState, Self::CommitmentProof), Self::Error>;
}

#[cgp_component {
  provider: AllClientStatesQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryAllClientStates<Counterparty>:
    HasClientIdType<Counterparty> + HasHeightType + HasAsyncErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_all_client_states(
        &self,
        height: &Self::Height,
    ) -> Result<Vec<(Self::ClientId, Counterparty::ClientState)>, Self::Error>;
}

#[cgp_component {
  provider: AllRawClientStatesQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryAllRawClientStates<Counterparty>:
    HasClientIdType<Counterparty> + HasHeightType + HasRawClientStateType + HasAsyncErrorType
{
    async fn query_all_raw_client_states(
        &self,
        height: &Self::Height,
    ) -> Result<Vec<(Self::ClientId, Self::RawClientState)>, Self::Error>;
}

#[async_trait]
pub trait CanQueryClientStateWithLatestHeight<Counterparty>:
    HasClientIdType<Counterparty> + HasHeightType + HasAsyncErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_client_state_with_latest_height(
        &self,
        tag: PhantomData<Counterparty>,
        client_id: &Self::ClientId,
    ) -> Result<Counterparty::ClientState, Self::Error>;
}

impl<Chain, Counterparty> CanQueryClientStateWithLatestHeight<Counterparty> for Chain
where
    Chain: CanQueryClientState<Counterparty> + CanQueryChainStatus,
    Counterparty: HasClientStateType<Chain>,
{
    async fn query_client_state_with_latest_height(
        &self,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let status = self.query_chain_status().await?;

        self.query_client_state(tag, client_id, Chain::chain_status_height(&status))
            .await
    }
}

#[async_trait]
pub trait CanQueryAllClientStatesWithLatestHeight<Counterparty>:
    HasClientIdType<Counterparty> + HasHeightType + HasAsyncErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_all_client_states_with_latest_height(
        &self,
    ) -> Result<Vec<(Self::ClientId, Counterparty::ClientState)>, Self::Error>;
}

impl<Chain, Counterparty> CanQueryAllClientStatesWithLatestHeight<Counterparty> for Chain
where
    Chain: CanQueryAllClientStates<Counterparty> + CanQueryChainStatus,
    Counterparty: HasClientStateType<Chain>,
{
    async fn query_all_client_states_with_latest_height(
        &self,
    ) -> Result<Vec<(Self::ClientId, Counterparty::ClientState)>, Chain::Error> {
        let status = self.query_chain_status().await?;

        self.query_all_client_states(Chain::chain_status_height(&status))
            .await
    }
}

impl<Chain, Counterparty, Components, Delegate> ClientStateQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasHeightType + HasClientIdType<Counterparty> + HasAsyncErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStateQuerier<Chain, Counterparty>,
{
    async fn query_client_state(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        Delegate::query_client_state(chain, tag, client_id, height).await
    }
}

impl<Chain, Counterparty, Components, Delegate> ClientStateWithProofsQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain:
        HasHeightType + HasClientIdType<Counterparty> + HasCommitmentProofType + HasAsyncErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStateWithProofsQuerier<Chain, Counterparty>,
{
    async fn query_client_state_with_proofs(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<(Counterparty::ClientState, Chain::CommitmentProof), Chain::Error> {
        Delegate::query_client_state_with_proofs(chain, tag, client_id, height).await
    }
}

impl<Chain, Counterparty, Components, Delegate> AllClientStatesQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasClientIdType<Counterparty> + HasHeightType + HasAsyncErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: AllClientStatesQuerier<Chain, Counterparty>,
{
    async fn query_all_client_states(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<(Chain::ClientId, Counterparty::ClientState)>, Chain::Error> {
        Delegate::query_all_client_states(chain, height).await
    }
}
