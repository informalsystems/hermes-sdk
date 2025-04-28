use hermes_ibc_components::traits::types::commitment::value::{
    CommitmentValueTypeComponent, ProvideCommitmentValueType,
};
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::tags::commitment::receive::ReceivePacket;
use hermes_ibc_components::types::tags::commitment::send::SendPacket;
use hermes_prelude::*;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;

#[cgp_provider(CommitmentValueTypeComponent)]
impl<Chain: Async, Counterparty: Async>
    ProvideCommitmentValueType<MockChain<Chain, Counterparty>, SendPacket> for MockChainComponents
{
    type CommitmentValue =
        IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>;
}

#[cgp_provider(CommitmentValueTypeComponent)]
impl<Chain: Async, Counterparty: Async>
    ProvideCommitmentValueType<MockChain<Chain, Counterparty>, ReceivePacket>
    for MockChainComponents
{
    type CommitmentValue =
        IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>;
}
