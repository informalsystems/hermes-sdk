use async_trait::async_trait;
use cgp_core::DelegateComponent;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{IncludeProof, QueryClientStateRequest, QueryHeight};
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerier;
use ibc_relayer_components::chain::traits::types::client_state::HasClientStateType;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::error::Error;
use crate::types::tendermint::TendermintClientState;

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

pub struct QueryCosmosClientStateFromChainHandle;

#[async_trait]
impl<Chain, Counterparty> ClientStateQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromChainHandle
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId> + HasBlockingChainHandle,
    Counterparty: HasClientStateType<Chain, ClientState = TendermintClientState>,
{
    async fn query_client_state(
        chain: &Chain,
        client_id: &ClientId,
    ) -> Result<TendermintClientState, Chain::Error> {
        let client_id = client_id.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (client_state, _) = chain_handle
                    .query_client_state(
                        QueryClientStateRequest {
                            client_id,
                            height: QueryHeight::Latest,
                        },
                        IncludeProof::No,
                    )
                    .map_err(Chain::raise_error)?;

                match client_state {
                    AnyClientState::Tendermint(client_state) => Ok(client_state),
                }
            })
            .await
    }
}
