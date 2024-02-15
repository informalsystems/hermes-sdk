use cgp_core::prelude::*;

use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::types::client_state::{HasClientStateFields, HasClientStateType};
use crate::chain::traits::types::consensus_state::{
    HasConsensusStateFields, HasConsensusStateType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

use super::chain_status::CanQueryChainStatus;
use super::client_state::CanQueryClientState;
use super::consensus_state::CanQueryConsensusState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ClientStatus {
    Frozen,
    Expired,
    Active,
}

#[async_trait]
pub trait CanQueryClientStatus<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasErrorType
    + CanQueryClientState<Counterparty>
    + CanQueryClientStateWithLatestHeight<Counterparty>
    + CanQueryChainStatus
    + CanQueryConsensusState<Counterparty>
where
    Counterparty: HasIbcChainTypes<Self>
        + HasClientStateType<Self>
        + HasClientStateFields<Self>
        + HasConsensusStateType<Self>
        + HasConsensusStateFields<Self>,
{
    async fn query_client_status(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<ClientStatus, Self::Error>;
}

#[async_trait]
impl<Chain, Counterparty> CanQueryClientStatus<Counterparty> for Chain
where
    Chain: HasIbcChainTypes<Counterparty>
        + HasErrorType
        + CanQueryClientState<Counterparty>
        + CanQueryClientStateWithLatestHeight<Counterparty>
        + CanQueryChainStatus
        + CanQueryConsensusState<Counterparty>,
    Counterparty: HasIbcChainTypes<Chain>
        + HasClientStateType<Chain>
        + HasClientStateFields<Chain>
        + HasConsensusStateType<Chain>
        + HasConsensusStateFields<Chain>,
{
    async fn query_client_status(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<ClientStatus, Self::Error> {
        let client_state = self
            .query_client_state_with_latest_height(client_id)
            .await?;
        // .wrap_err_with(|e| "Failed to query client state for client `{client_id}`")?;

        if Counterparty::client_state_is_frozen(&client_state) {
            return Ok(ClientStatus::Frozen);
        }

        let client_latest_height = Counterparty::client_state_latest_height(&client_state);

        let latest_consensus_state = self
            .query_consensus_state(client_id, client_latest_height)
            .await?;
        // .wrap_err_with(|| {
        //     format!("Failed to query consensus state at height {client_latest_height}")
        // })?;

        let latest_consensus_state_timestamp =
            Counterparty::consensus_state_timestamp(&latest_consensus_state);

        let chain_status = self.query_chain_status().await?;
        // .wrap_err("Failed to query chain status")?;

        let current_network_time = Self::chain_status_timestamp(&chain_status);

        let elapsed = Self::timestamp_duration_since(
            latest_consensus_state_timestamp.as_ref(),
            current_network_time,
        );

        let has_expired = elapsed.map_or(false, |elapsed| {
            Counterparty::client_state_has_expired(&client_state, elapsed)
        });

        if has_expired {
            Ok(ClientStatus::Expired)
        } else {
            Ok(ClientStatus::Active)
        }
    }
}
