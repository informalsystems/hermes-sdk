use cgp_core::prelude::*;

use crate::chain::traits::queries::chain_status::CanQueryChainStatus;
use crate::chain::traits::types::client_state::HasClientStateType;
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
