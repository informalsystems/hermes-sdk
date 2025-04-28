use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::sync::Arc;

use hermes_ibc_components::traits::commitment::verify::{
    CommitmentVerifier, CommitmentVerifierComponent,
};
use hermes_ibc_components::types::packet::IbcPacket;
use hermes_ibc_components::types::tags::commitment::receive::ReceivePacket;
use hermes_ibc_components::types::tags::commitment::send::SendPacket;
use hermes_prelude::*;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::{MockChain, MockChainState};
use crate::types::commitment::path::{
    MockReceivePacketCommitmentPath, MockSendPacketCommitmentPath,
};
use crate::types::commitment::proof::MockCommitmentProof;

#[cgp_provider(CommitmentVerifierComponent)]
impl<Chain: Async, Counterparty: Async>
    CommitmentVerifier<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>, SendPacket>
    for MockChainComponents
{
    fn verify_commitment(
        consensus_state: &Arc<MockChainState<Chain, Counterparty>>,
        commitment_path: &MockSendPacketCommitmentPath<Chain, Counterparty>,
        packet: &IbcPacket<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>>,
        _proof: &MockCommitmentProof<Chain, Counterparty>,
    ) -> Result<(), String> {
        let send_packets = consensus_state
            .sent_packets
            .get(&(
                commitment_path.src_channel_id.clone(),
                commitment_path.dst_channel_id.clone(),
            ))
            .ok_or_else(|| "channel has no sent packet".to_owned())?;

        let committed_packet = send_packets
            .get(&commitment_path.nonce)
            .ok_or_else(|| "packet was not sent".to_owned())?;

        if packet != committed_packet {
            return Err("packet commitment mismatch".to_owned());
        }

        Ok(())
    }
}

#[cgp_provider(CommitmentVerifierComponent)]
impl<Chain: Async, Counterparty: Async>
    CommitmentVerifier<
        MockChain<Chain, Counterparty>,
        MockChain<Counterparty, Chain>,
        ReceivePacket,
    > for MockChainComponents
{
    fn verify_commitment(
        consensus_state: &Arc<MockChainState<Chain, Counterparty>>,
        commitment_path: &MockReceivePacketCommitmentPath<Chain, Counterparty>,
        packet: &IbcPacket<MockChain<Counterparty, Chain>, MockChain<Chain, Counterparty>>,
        _proof: &MockCommitmentProof<Chain, Counterparty>,
    ) -> Result<(), String> {
        let received_packets = consensus_state
            .received_packets
            .get(&(
                commitment_path.dst_channel_id.clone(),
                commitment_path.src_channel_id.clone(),
            ))
            .ok_or_else(|| "channel has no received packet".to_owned())?;

        let committed_packet = received_packets
            .get(&commitment_path.nonce)
            .ok_or_else(|| "packet was not received".to_owned())?;

        if packet != committed_packet {
            return Err("packet commitment mismatch".to_owned());
        }

        Ok(())
    }
}
