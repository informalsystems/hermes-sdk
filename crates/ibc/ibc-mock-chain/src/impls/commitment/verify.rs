use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::sync::Arc;
use cgp::core::Async;
use hermes_ibc_components::traits::commitment::verify::CommitmentVerifier;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::{MockChain, MockChainState};
use crate::types::commitment::path::MockCommitmentPath;
use crate::types::commitment::proof::MockCommitmentProof;
use crate::types::commitment::value::MockCommitmentValue;
use crate::types::tagged::Tagged;

impl<Chain: Async, Counterparty: Async, Tag>
    CommitmentVerifier<MockChain<Chain, Counterparty>, MockChain<Counterparty, Chain>, Tag>
    for MockChainComponents
{
    fn verify_commitment(
        consensus_state: &Arc<MockChainState<Chain, Counterparty>>,
        commitment_path: &MockCommitmentPath<Chain, Counterparty>,
        commitment_value: &MockCommitmentValue<Chain, Counterparty>,
        _proof: &Tagged<Chain, Counterparty, MockCommitmentProof>,
    ) -> Result<(), String> {
        match (commitment_path, commitment_value) {
            (
                MockCommitmentPath::ReceivePacket {
                    src_channel_id,
                    dst_channel_id,
                    nonce,
                },
                MockCommitmentValue::ReceivePacket(packet),
            ) => {
                let received_packets = consensus_state
                    .received_packets
                    .get(&(dst_channel_id.clone(), src_channel_id.clone()))
                    .ok_or_else(|| "channel has no received packet".to_owned())?;

                let committed_packet = received_packets
                    .get(nonce)
                    .ok_or_else(|| "packet was not received".to_owned())?;

                if packet != committed_packet {
                    return Err("packet commitment mismatch".to_owned());
                }

                Ok(())
            }
            (
                MockCommitmentPath::SendPacket {
                    src_channel_id,
                    dst_channel_id,
                    nonce,
                },
                MockCommitmentValue::SendPacket(packet),
            ) => {
                let send_packets = consensus_state
                    .sent_packets
                    .get(&(src_channel_id.clone(), dst_channel_id.clone()))
                    .ok_or_else(|| "channel has no sent packet".to_owned())?;

                let committed_packet = send_packets
                    .get(nonce)
                    .ok_or_else(|| "packet was not sent".to_owned())?;

                if packet != committed_packet {
                    return Err("packet commitment mismatch".to_owned());
                }

                Ok(())
            }
            _ => Err("invalid commitment".to_owned()),
        }
    }
}
