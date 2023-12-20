use async_trait::async_trait;
use cgp_core::DelegateComponent;
use cosmos_client_components::components::query_consensus_state::QueryCosmosConsensusStateFromChainHandle;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerier;
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::height::HasHeightType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosConsensusStateQuerier;

#[async_trait]
impl<Chain, Counterparty, Delegate> ConsensusStateQuerier<CosmosChain<Chain>, Counterparty>
    for DelegateCosmosConsensusStateQuerier
where
    Chain: ChainHandle,
    Counterparty: HasConsensusStateType<CosmosChain<Chain>> + HasHeightType,
    Delegate: ConsensusStateQuerier<CosmosChain<Chain>, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_consensus_state(
        chain: &CosmosChain<Chain>,
        client_id: &ClientId,
        height: &Counterparty::Height,
    ) -> Result<Counterparty::ConsensusState, Error> {
        Delegate::query_consensus_state(chain, client_id, height).await
    }
}

impl<Counterparty> DelegateComponent<CosmosChain<Counterparty>>
    for DelegateCosmosConsensusStateQuerier
where
    Counterparty: ChainHandle,
{
    type Delegate = QueryCosmosConsensusStateFromChainHandle;
}
