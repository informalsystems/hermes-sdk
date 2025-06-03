use core::marker::PhantomData;

use hermes_core::chain_components::traits::{
    CanMeasureTime, CanQueryChainStatus, CanQueryClientStateWithLatestHeight,
    CanQueryConsensusStateWithLatestHeight, ClientStatus, ClientStatusQuerier,
    ClientStatusQuerierComponent, HasClientStateFields, HasClientStatusType,
    HasConsensusStateFields,
};
use hermes_core::relayer_components::chain::traits::HasIbcChainTypes;
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ClientId;

pub struct QueryCosmosClientStatus;

#[cgp_provider(ClientStatusQuerierComponent)]
impl<Chain, Counterparty> ClientStatusQuerier<Chain, Counterparty> for QueryCosmosClientStatus
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + CanQueryConsensusStateWithLatestHeight<Counterparty>
        + CanQueryClientStateWithLatestHeight<Counterparty>
        + CanQueryChainStatus
        + CanMeasureTime,
    Counterparty: HasClientStatusType<Chain, ClientStatus = ClientStatus>
        + HasClientStateFields<Chain>
        + HasConsensusStateFields<Chain>,
{
    async fn query_client_status(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &ClientId,
    ) -> Result<ClientStatus, Chain::Error> {
        let client_state = chain
            .query_client_state_with_latest_height(PhantomData, client_id)
            .await?;

        if Counterparty::client_state_is_frozen(&client_state) {
            return Ok(ClientStatus::Frozen);
        }

        let client_latest_height = Counterparty::client_state_latest_height(&client_state);

        let latest_consensus_state = chain
            .query_consensus_state_with_latest_height(PhantomData, client_id, &client_latest_height)
            .await?;

        let latest_consensus_state_timestamp =
            Counterparty::consensus_state_timestamp(&latest_consensus_state);

        let chain_status = chain.query_chain_status().await?;

        let current_network_time = Chain::chain_status_time(&chain_status);

        let elapsed =
            Chain::duration_since(&latest_consensus_state_timestamp, current_network_time);

        if elapsed
            .is_some_and(|elapsed| Counterparty::client_state_has_expired(&client_state, elapsed))
        {
            return Ok(ClientStatus::Expired);
        }

        Ok(ClientStatus::Active)
    }
}
