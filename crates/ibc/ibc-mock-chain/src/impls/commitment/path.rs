use alloc::string::String;

use cgp::prelude::*;
use hermes_ibc_components::traits::commitment::path::receive_packet::{
    ReceivePacketCommitmentPathBuilder, ReceivePacketCommitmentPathBuilderComponent,
};
use hermes_ibc_components::traits::commitment::path::send_packet::{
    SendPacketCommitmentPathBuilder, SendPacketCommitmentPathBuilderComponent,
};

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::channel_id::MockChannelId;
use crate::types::commitment::path::{
    MockReceivePacketCommitmentPath, MockSendPacketCommitmentPath,
};
use crate::types::nonce::MockNonce;
use crate::types::tagged::Tagged;

#[cgp_provider(ReceivePacketCommitmentPathBuilderComponent)]
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
    ) -> Result<MockReceivePacketCommitmentPath<Chain, Counterparty>, String> {
        Ok(MockReceivePacketCommitmentPath {
            src_channel_id: src_channel_id.clone(),
            dst_channel_id: dst_channel_id.clone(),
            nonce: nonce.clone(),
        })
    }
}

#[cgp_provider(SendPacketCommitmentPathBuilderComponent)]
impl<Chain: Async, Counterparty: Async>
    SendPacketCommitmentPathBuilder<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    fn build_send_packet_commitment_path(
        src_channel_id: &Tagged<Chain, Counterparty, MockChannelId>,
        dst_channel_id: &Tagged<Counterparty, Chain, MockChannelId>,
        nonce: &Tagged<Chain, Counterparty, MockNonce>,
    ) -> Result<MockSendPacketCommitmentPath<Chain, Counterparty>, String> {
        Ok(MockSendPacketCommitmentPath {
            src_channel_id: src_channel_id.clone(),
            dst_channel_id: dst_channel_id.clone(),
            nonce: nonce.clone(),
        })
    }
}
