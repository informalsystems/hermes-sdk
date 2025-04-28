use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::sync::Arc;

use hermes_chain_type_components::traits::{
    ConsensusStateTypeComponent, ProvideConsensusStateType,
};
use hermes_ibc_components::traits::queries::consensus_state::{
    ConsensusStateQuerier, ConsensusStateQuerierComponent,
};
use hermes_prelude::*;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::{MockChain, MockChainState};
use crate::types::client_id::MockClientId;
use crate::types::height::MockHeight;
use crate::types::tagged::Tagged;

#[cgp_provider(ConsensusStateTypeComponent)]
impl<A: Async, B: Async, Counterparty> ProvideConsensusStateType<MockChain<A, B>, Counterparty>
    for MockChainComponents
{
    type ConsensusState = Arc<MockChainState<A, B>>;
}

#[cgp_provider(ConsensusStateQuerierComponent)]
impl<Chain: Async, Counterparty: Async>
    ConsensusStateQuerier<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn query_consensus_state(
        chain: &MockChain<Chain, Counterparty>,
        client_id: &Tagged<Chain, Counterparty, MockClientId>,
        height: &Tagged<Counterparty, Chain, MockHeight>,
    ) -> Result<Arc<MockChainState<Counterparty, Chain>>, String> {
        let state = chain.pending_state.mock_chain_state();

        let consensus_states = state
            .consensus_states
            .get(client_id)
            .ok_or_else(|| "client not found".to_owned())?;

        let consensus_state = consensus_states
            .get(height)
            .ok_or_else(|| "consensus state not found at given height".to_owned())?;

        Ok(consensus_state.clone())
    }
}
