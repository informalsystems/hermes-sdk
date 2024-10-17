use cgp::core::Async;
use hermes_ibc_components::traits::types::commitment::value::ProvideCommitmentValueType;
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::tags::commitment::receive::ReceivePacket;
use hermes_ibc_components::types::tags::commitment::send::SendPacket;

use crate::contexts::chain::MockChain;

pub struct UseMockCommitmentValue;

impl<Chain: Async, Counterparty: Async>
    ProvideCommitmentValueType<MockChain<Chain, Counterparty>, SendPacket>
    for UseMockCommitmentValue
{
    type CommitmentValue =
        IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>;
}

impl<Chain: Async, Counterparty: Async>
    ProvideCommitmentValueType<MockChain<Chain, Counterparty>, ReceivePacket>
    for UseMockCommitmentValue
{
    type CommitmentValue =
        IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>;
}
