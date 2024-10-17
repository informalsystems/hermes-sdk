use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::sync::Arc;
use cgp::core::Async;
use hermes_chain_type_components::traits::types::ibc::consensus_state::ProvideConsensusStateType;
use hermes_ibc_components::traits::queries::consensus_state::ConsensusStateQuerier;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::{MockChain, MockChainState};
use crate::types::client_id::MockClientId;
use crate::types::height::MockHeight;
use crate::types::tagged::Tagged;

impl<A: Async, B: Async, Counterparty> ProvideConsensusStateType<MockChain<A, B>, Counterparty>
    for MockChainComponents
{
    type ConsensusState = Arc<MockChainState<A, B>>;
}

impl<Chain: Async, Counterparty: Async>
    ConsensusStateQuerier<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn query_consensus_state(
        chain: &MockChain<Chain, Counterparty>,
        client_id: &Tagged<Chain, Counterparty, MockClientId>,
        height: &Tagged<Counterparty, Chain, MockHeight>,
    ) -> Result<Arc<MockChainState<Counterparty, Chain>>, String> {
        let mut lock = chain.pending_state.lock().await;
        let state = lock.mock_chain_state_mut();

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
