use alloc::string::String;

use cgp::prelude::*;
use hermes_ibc_components::traits::commitment::value::receive_packet::{
    ReceivePacketCommitmentValueBuilder, ReceivePacketCommitmentValueBuilderComponent,
};
use hermes_ibc_components::traits::commitment::value::send_packet::{
    SendPacketCommitmentValueBuilder, SendPacketCommitmentValueBuilderComponent,
};
use hermes_ibc_components::types::packet::IbcPacket;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;

#[cgp_provider(SendPacketCommitmentValueBuilderComponent)]
impl<Chain: Async, Counterparty: Async>
    SendPacketCommitmentValueBuilder<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>
    for MockChainComponents
{
    fn build_send_packet_commitment_value(
        packet: &IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>,
    ) -> Result<IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>, String>
    {
        Ok(packet.clone())
    }
}

#[cgp_provider(ReceivePacketCommitmentValueBuilderComponent)]
impl<Chain: Async, Counterparty: Async>
    ReceivePacketCommitmentValueBuilder<
        MockChain<Chain, Counterparty>,
        MockChain<Counterparty, Chain>,
    > for MockChainComponents
{
    fn build_receive_packet_commitment_value(
        packet: &IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>,
    ) -> Result<IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>, String>
    {
        Ok(packet.clone())
    }
}
