use alloc::vec::Vec;
use cgp_core::prelude::*;

use crate::chain::traits::queries::chain_status::CanQueryChainStatus;
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(ClientStatesQuerierComponent, ClientStatesQuerier<Chain>)]
#[async_trait]
pub trait CanQueryClientStates<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_client_states(
        &self,
        height: &Self::Height,
    ) -> Result<Vec<(Self::ClientId, Counterparty::ClientState)>, Self::Error>;
}

#[async_trait]
pub trait CanQueryClientStatesWithLatestHeight<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_client_states_with_latest_height(
        &self,
    ) -> Result<Vec<(Self::ClientId, Counterparty::ClientState)>, Self::Error>;
}

impl<Chain, Counterparty> CanQueryClientStatesWithLatestHeight<Counterparty> for Chain
where
    Chain: CanQueryClientStates<Counterparty> + CanQueryChainStatus,
    Counterparty: HasClientStateType<Chain>,
{
    async fn query_client_states_with_latest_height(
        &self,
    ) -> Result<Vec<(Self::ClientId, Counterparty::ClientState)>, Chain::Error> {
        let status = self.query_chain_status().await?;

        self.query_client_states(Chain::chain_status_height(&status))
            .await
    }
}

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
