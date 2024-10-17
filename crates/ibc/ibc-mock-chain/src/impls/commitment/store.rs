use alloc::borrow::ToOwned;
use alloc::string::String;
use cgp::core::Async;
use hermes_ibc_components::traits::commitment::store::CommitmentStorage;
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::tags::commitment::receive::ReceivePacket;
use hermes_ibc_components::types::tags::commitment::send::SendPacket;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::commitment::path::{
    MockReceivePacketCommitmentPath, MockSendPacketCommitmentPath,
};

impl<Chain: Async, Counterparty: Async>
    CommitmentStorage<MockChain<Chain, Counterparty>, SendPacket> for MockChainComponents
{
    async fn store_commitment(
        chain: &MockChain<Chain, Counterparty>,
        commitment_path: &MockSendPacketCommitmentPath<Chain, Counterparty>,
        packet: &IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>,
    ) -> Result<(), String> {
        let mut lock = chain.pending_state.lock().await;
        let state = lock.mock_chain_state_mut();

        let sent_packets = state
            .sent_packets
            .entry((
                commitment_path.src_channel_id.clone(),
                commitment_path.dst_channel_id.clone(),
            ))
            .or_default();

        if sent_packets.contains_key(&commitment_path.nonce) {
            return Err("commitment already exist".to_owned());
        }

        sent_packets.insert(commitment_path.nonce.clone(), packet.clone());

        Ok(())
    }
}

impl<Chain: Async, Counterparty: Async>
    CommitmentStorage<MockChain<Chain, Counterparty>, ReceivePacket> for MockChainComponents
{
    async fn store_commitment(
        chain: &MockChain<Chain, Counterparty>,
        commitment_path: &MockReceivePacketCommitmentPath<Chain, Counterparty>,
        packet: &IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>,
    ) -> Result<(), String> {
        let mut lock = chain.pending_state.lock().await;
        let state = lock.mock_chain_state_mut();

        let received_packets = state
            .received_packets
            .entry((
                commitment_path.dst_channel_id.clone(),
                commitment_path.src_channel_id.clone(),
            ))
            .or_default();

        if received_packets.contains_key(&commitment_path.nonce) {
            return Err("commitment already exist".to_owned());
        }

        received_packets.insert(commitment_path.nonce.clone(), packet.clone());

        Ok(())
    }
}
