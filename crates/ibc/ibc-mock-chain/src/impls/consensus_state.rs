use alloc::sync::Arc;
use cgp::core::Async;
use hermes_chain_type_components::traits::types::ibc::consensus_state::ProvideConsensusStateType;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::{MockChain, MockChainState};

impl<Chain: Async, Counterparty: Async>
    ProvideConsensusStateType<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    type ConsensusState = Arc<MockChainState<Chain, Counterparty>>;
}
