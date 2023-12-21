use async_trait::async_trait;
use cgp_core::DelegateComponent;
use hermes_cosmos_client_components::components::query_consensus_state::QueryCosmosConsensusStateFromChainHandle;
use hermes_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerier;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosConsensusStateQuerier;

#[async_trait]
impl<Counterparty, Delegate> ConsensusStateQuerier<CosmosChain, Counterparty>
    for DelegateCosmosConsensusStateQuerier
where
    Counterparty: HasConsensusStateType<CosmosChain> + HasHeightType,
    Delegate: ConsensusStateQuerier<CosmosChain, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_consensus_state(
        chain: &CosmosChain,
        client_id: &ClientId,
        height: &Counterparty::Height,
    ) -> Result<Counterparty::ConsensusState, Error> {
        Delegate::query_consensus_state(chain, client_id, height).await
    }
}

impl DelegateComponent<CosmosChain> for DelegateCosmosConsensusStateQuerier {
    type Delegate = QueryCosmosConsensusStateFromChainHandle;
}
