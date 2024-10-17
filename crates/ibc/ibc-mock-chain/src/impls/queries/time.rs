use cgp::core::Async;
use hermes_ibc_components::traits::fields::timeout::TimeoutTimeComparer;
use hermes_ibc_components::traits::queries::time::CurrentTimeQuerier;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::height::MockHeight;
use crate::types::tagged::Tagged;

impl<Chain: Async, Counterparty: Async> CurrentTimeQuerier<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    async fn get_current_time(
        chain: &MockChain<Chain, Counterparty>,
    ) -> Tagged<Chain, Counterparty, MockHeight> {
        let mut lock = chain.pending_state.lock().await;
        let state = lock.mock_chain_state_mut();

        state.current_height.clone()
    }
}

impl<Chain: Async, Counterparty: Async>
    TimeoutTimeComparer<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    fn is_packet_timed_out(
        current_time: &Tagged<Chain, Counterparty, MockHeight>,
        timeout: &Tagged<Chain, Counterparty, MockHeight>,
    ) -> bool {
        current_time.value > timeout.value
    }
}
