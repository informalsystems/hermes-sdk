use alloc::vec::Vec;

use cgp_core::prelude::*;

use crate::chain::traits::queries::chain_status::CanQueryChainStatus;
use crate::chain::traits::types::client_state::{HasClientStateType, HasRawClientStateType};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

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

#[derive_component(ClientStateBytesQuerierComponent, ClientStateBytesQuerier<Chain>)]
#[async_trait]
pub trait CanQueryClientStateBytes<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
{
    async fn query_client_state_bytes(
        &self,
        client_id: &Self::ClientId,
        height: &Self::Height,
    ) -> Result<Vec<u8>, Self::Error>;
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

#[derive_component(AllClientStatesBytesQuerierComponent, AllClientStatesBytesQuerier<Chain>)]
#[async_trait]
pub trait CanQueryAllClientStatesBytes<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
{
    async fn query_all_client_states_bytes(
        &self,
        height: &Self::Height,
    ) -> Result<Vec<(Self::ClientId, Vec<u8>)>, Self::Error>;
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
