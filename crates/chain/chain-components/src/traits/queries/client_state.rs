use alloc::vec::Vec;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

use crate::traits::queries::chain_status::CanQueryChainStatus;
use crate::traits::types::client_state::{HasClientStateType, HasRawClientStateType};
use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::proof::HasCommitmentProofType;

#[derive_component(ClientStateQuerierComponent, ClientStateQuerier<Chain>)]
#[async_trait]
pub trait CanQueryClientState<Counterparty>: HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_client_state(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Counterparty::ClientState, Self::Error>;
}

#[derive_component(ClientStateWithProofsQuerierComponent, ClientStateWithProofsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryClientStateWithProofs<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasCommitmentProofType + HasErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_client_state_with_proofs(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<(Counterparty::ClientState, Self::CommitmentProof), Self::Error>;
}

#[derive_component(RawClientStateQuerierComponent, RawClientStateQuerier<Chain>)]
#[async_trait]
pub trait CanQueryRawClientState<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasRawClientStateType + HasErrorType
{
    async fn query_raw_client_state(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Self::RawClientState, Self::Error>;
}

#[derive_component(RawClientStateWithProofsQuerierComponent, RawClientStateWithProofsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryRawClientStateWithProofs<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasRawClientStateType + HasCommitmentProofType + HasErrorType
{
    async fn query_raw_client_state_with_proofs(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<(Self::RawClientState, Self::CommitmentProof), Self::Error>;
}

#[derive_component(AllClientStatesQuerierComponent, AllClientStatesQuerier<Chain>)]
#[async_trait]
pub trait CanQueryAllClientStates<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_all_client_states(
        &self,
        height: &Self::Height,
    ) -> Result<Vec<(Self::ClientId, Counterparty::ClientState)>, Self::Error>;
}

#[derive_component(AllRawClientStatesQuerierComponent, AllRawClientStatesQuerier<Chain>)]
#[async_trait]
pub trait CanQueryAllRawClientStates<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasRawClientStateType + HasErrorType
{
    async fn query_all_raw_client_states(
        &self,
        height: &Self::Height,
    ) -> Result<Vec<(Self::ClientId, Self::RawClientState)>, Self::Error>;
}

#[async_trait]
pub trait CanQueryClientStateWithLatestHeight<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_client_state_with_latest_height(
        &self,
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
        client_id: &Chain::ClientId,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        let status = self.query_chain_status().await?;

        self.query_client_state(client_id, Chain::chain_status_height(&status))
            .await
    }
}

#[async_trait]
pub trait CanQueryAllClientStatesWithLatestHeight<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
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
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStateQuerier<Chain, Counterparty>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        Delegate::query_client_state(chain, client_id, height).await
    }
}

impl<Chain, Counterparty, Components, Delegate> ClientStateWithProofsQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasCommitmentProofType + HasErrorType,
    Counterparty: HasClientStateType<Chain>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStateWithProofsQuerier<Chain, Counterparty>,
{
    async fn query_client_state_with_proofs(
        chain: &Chain,
        client_id: &Chain::ClientId,
        height: &Chain::Height,
    ) -> Result<(Counterparty::ClientState, Chain::CommitmentProof), Chain::Error> {
        Delegate::query_client_state_with_proofs(chain, client_id, height).await
    }
}

impl<Chain, Counterparty, Components, Delegate> AllClientStatesQuerier<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
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
