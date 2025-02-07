use alloc::string::String;

use cgp::prelude::*;
use hermes_ibc_components::traits::queries::recv_packet_commitment::{
    HasPacketReceivedQuerier, HasPacketReceivedQuerierComponent,
};

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::channel_id::MockChannelId;
use crate::types::nonce::MockNonce;
use crate::types::tagged::Tagged;

#[cgp_provider(HasPacketReceivedQuerierComponent)]
impl<Chain: Async, Counterparty: Async>
    HasPacketReceivedQuerier<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    async fn query_has_packet_received(
        chain: &MockChain<Chain, Counterparty>,
        src_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        dst_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        nonce: &Tagged<Counterparty, Chain, MockNonce>,
    ) -> Result<bool, String> {
        let state = chain.pending_state.mock_chain_state();

        let m_received_packets = state
            .received_packets
            .get(&(dst_channel_id.clone(), src_channel_id.clone()));

        let received = if let Some(received_packets) = m_received_packets {
            received_packets.contains_key(nonce)
        } else {
            false
        };

        Ok(received)
    }
}
