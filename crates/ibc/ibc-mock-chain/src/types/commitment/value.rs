use cgp::core::Async;
use hermes_ibc_components::traits::types::commitment::value::ProvideCommitmentValueType;
use hermes_ibc_components::types::packet::IbcPacket;

use crate::contexts::chain::MockChain;

pub enum MockCommitmentValue<Chain: Async, Counterparty: Async> {
    SendPacket(IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>),
    ReceivePacket(IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>),
}

pub struct UseMockCommitmentValue;

impl<Chain: Async, Counterparty: Async> ProvideCommitmentValueType<MockChain<Chain, Counterparty>>
    for UseMockCommitmentValue
{
    type CommitmentValue = MockCommitmentValue<Chain, Counterparty>;
}
