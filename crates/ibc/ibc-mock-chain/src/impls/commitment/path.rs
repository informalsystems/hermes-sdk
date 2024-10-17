use alloc::string::String;
use cgp::core::Async;
use hermes_ibc_components::traits::commitment::path::receive_packet::ReceivePacketCommitmentPathBuilder;
use hermes_ibc_components::traits::commitment::path::send_packet::SendPacketCommitmentPathBuilder;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::channel_id::MockChannelId;
use crate::types::commitment::path::MockCommitmentPath;
use crate::types::nonce::MockNonce;
use crate::types::tagged::Tagged;

impl<Chain: Async, Counterparty: Async>
    ReceivePacketCommitmentPathBuilder<
        MockChain<Chain, Counterparty>,
        MockChain<Counterparty, Chain>,
    > for MockChainComponents
{
    fn build_receive_packet_commitment_path(
        src_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        dst_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        nonce: &Tagged<Counterparty, Chain, MockNonce>,
    ) -> Result<MockCommitmentPath<Chain, Counterparty>, String> {
        Ok(MockCommitmentPath::ReceivePacket {
            src_channel_id: src_channel_id.clone(),
            dst_channel_id: dst_channel_id.clone(),
            nonce: nonce.clone(),
        })
    }
}

impl<Chain: Async, Counterparty: Async>
    SendPacketCommitmentPathBuilder<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    fn build_send_packet_commitment_path(
        src_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        dst_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        nonce: &Tagged<Chain, Counterparty, MockNonce>,
    ) -> Result<MockCommitmentPath<Chain, Counterparty>, String> {
        Ok(MockCommitmentPath::SendPacket {
            src_channel_id: src_channel_id.clone(),
            dst_channel_id: dst_channel_id.clone(),
            nonce: nonce.clone(),
        })
    }
}
