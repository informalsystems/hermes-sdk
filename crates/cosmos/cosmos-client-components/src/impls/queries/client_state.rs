use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerier, ClientStateWithHeightQuerier,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{IncludeProof, QueryClientStateRequest, QueryHeight};
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::tendermint::TendermintClientState;

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

#[async_trait]
impl<Chain, Counterparty> ClientStateWithHeightQuerier<Chain, Counterparty>
    for QueryCosmosClientStateFromChainHandle
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasBlockingChainHandle,
    Counterparty: HasClientStateType<Chain, ClientState = TendermintClientState>,
{
    async fn query_client_state_with_height(
        chain: &Chain,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<TendermintClientState, Chain::Error> {
        let client_id = client_id.clone();
        let height = height.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (client_state, _) = chain_handle
                    .query_client_state(
                        QueryClientStateRequest {
                            client_id,
                            height: QueryHeight::Specific(height),
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
