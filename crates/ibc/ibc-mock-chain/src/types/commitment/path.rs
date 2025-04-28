use hermes_ibc_components::traits::types::commitment::path::{
    CommitmentPathTypeComponent, ProvideCommitmentPathType,
};
use hermes_ibc_components::types::tags::commitment::receive::ReceivePacket;
use hermes_ibc_components::types::tags::commitment::send::SendPacket;
use hermes_prelude::*;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::channel_id::MockChannelId;
use crate::types::nonce::MockNonce;
use crate::types::tagged::Tagged;

pub struct MockSendPacketCommitmentPath<Chain, Counterparty> {
    pub src_channel_id: Tagged<Chain, Counterparty, MockChannelId>,
    pub dst_channel_id: Tagged<Counterparty, Chain, MockChannelId>,
    pub nonce: Tagged<Chain, Counterparty, MockNonce>,
}

pub struct MockReceivePacketCommitmentPath<Chain, Counterparty> {
    pub src_channel_id: Tagged<Counterparty, Chain, MockChannelId>,
    pub dst_channel_id: Tagged<Chain, Counterparty, MockChannelId>,
    pub nonce: Tagged<Counterparty, Chain, MockNonce>,
}

#[cgp_provider(CommitmentPathTypeComponent)]
impl<Chain: Async, Counterparty: Async>
    ProvideCommitmentPathType<MockChain<Chain, Counterparty>, SendPacket> for MockChainComponents
{
    type CommitmentPath = MockSendPacketCommitmentPath<Chain, Counterparty>;
}

#[cgp_provider(CommitmentPathTypeComponent)]
impl<Chain: Async, Counterparty: Async>
    ProvideCommitmentPathType<MockChain<Chain, Counterparty>, ReceivePacket>
    for MockChainComponents
{
    type CommitmentPath = MockReceivePacketCommitmentPath<Chain, Counterparty>;
}
