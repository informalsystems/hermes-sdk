use alloc::string::String;

use cgp::prelude::*;
use hermes_ibc_components::traits::nonce::{PacketNonceAllocator, PacketNonceAllocatorComponent};

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::channel_id::MockChannelId;
use crate::types::nonce::MockNonce;
use crate::types::tagged::Tagged;

#[cgp_provider(PacketNonceAllocatorComponent)]
impl<Chain: Async, Counterparty: Async>
    PacketNonceAllocator<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn allocate_packet_nonce(
        chain: &mut MockChain<Chain, Counterparty>,
        src_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        dst_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
    ) -> Result<Tagged<Chain, Counterparty, MockNonce>, String> {
        let state = chain.pending_state.mock_chain_state_mut();

        let next_nonce = state
            .next_nonce
            .entry((src_channel_id.clone(), dst_channel_id.clone()))
            .or_default();

        let nonce = next_nonce.clone();

        next_nonce.value.value += 1;

        Ok(nonce)
    }
}
