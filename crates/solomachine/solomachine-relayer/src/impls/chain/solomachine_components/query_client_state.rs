use async_trait::async_trait;
use hermes_cosmos_client_components::types::tendermint::TendermintClientState;
use hermes_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerier;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;

pub struct QueryCosmosClientStateFromSolomachine;

#[async_trait]
impl<Chain, Counterparty> ClientStateQuerier<SolomachineChain<Chain>, Counterparty>
    for QueryCosmosClientStateFromSolomachine
where
    Chain: Solomachine,
    Counterparty: HasClientStateType<SolomachineChain<Chain>, ClientState = TendermintClientState>,
{
    async fn query_client_state(
        chain: &SolomachineChain<Chain>,
        client_id: &ClientId,
    ) -> Result<TendermintClientState, Chain::Error> {
        chain.chain.query_client_state(client_id).await
    }
}
