use async_trait::async_trait;
use cgp_core::DelegateComponent;
use hermes_cosmos_client_components::components::query_client_state::QueryCosmosClientStateFromChainHandle;
use hermes_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerier;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosClientStateQuerier;

#[async_trait]
impl<Chain, Counterparty, Delegate> ClientStateQuerier<CosmosChain<Chain>, Counterparty>
    for DelegateCosmosClientStateQuerier
where
    Chain: ChainHandle,
    Counterparty: HasClientStateType<CosmosChain<Chain>>,
    Delegate: ClientStateQuerier<CosmosChain<Chain>, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_client_state(
        chain: &CosmosChain<Chain>,
        client_id: &ClientId,
    ) -> Result<Counterparty::ClientState, Error> {
        Delegate::query_client_state(chain, client_id).await
    }
}

impl<Counterparty> DelegateComponent<CosmosChain<Counterparty>> for DelegateCosmosClientStateQuerier
where
    Counterparty: ChainHandle,
{
    type Delegate = QueryCosmosClientStateFromChainHandle;
}
