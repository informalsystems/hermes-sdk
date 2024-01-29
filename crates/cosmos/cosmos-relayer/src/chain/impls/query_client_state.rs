use cgp_core::prelude::*;
use hermes_cosmos_client_components::components::ibc_client::CosmosIbcClientComponents;
use hermes_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerier;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosClientStateQuerier;

#[async_trait]
impl<Counterparty, Delegate> ClientStateQuerier<CosmosChain, Counterparty>
    for DelegateCosmosClientStateQuerier
where
    Counterparty: HasClientStateType<CosmosChain>,
    Delegate: ClientStateQuerier<CosmosChain, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_client_state(
        chain: &CosmosChain,
        client_id: &ClientId,
    ) -> Result<Counterparty::ClientState, Error> {
        Delegate::query_client_state(chain, client_id).await
    }
}

impl DelegateComponent<CosmosChain> for DelegateCosmosClientStateQuerier {
    type Delegate = CosmosIbcClientComponents;
}
