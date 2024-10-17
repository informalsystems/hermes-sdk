use alloc::borrow::ToOwned;
use alloc::string::String;
use cgp::core::Async;
use hermes_ibc_components::traits::commitment::store::CommitmentStorage;

use crate::components::chain::MockChainComponents;
use crate::contexts::chain::MockChain;
use crate::types::commitment::path::MockCommitmentPath;
use crate::types::commitment::value::MockCommitmentValue;

impl<Chain: Async, Counterparty: Async> CommitmentStorage<MockChain<Chain, Counterparty>>
    for MockChainComponents
{
    async fn store_commitment(
        chain: &MockChain<Chain, Counterparty>,
        commitment_path: &MockCommitmentPath<Chain, Counterparty>,
        commitment_value: &MockCommitmentValue<Chain, Counterparty>,
    ) -> Result<(), String> {
        let mut lock = chain.pending_state.lock().await;
        let state = lock.mock_chain_state_mut();

        match (commitment_path, commitment_value) {
            (
                MockCommitmentPath::ReceivePacket {
                    src_channel_id,
                    dst_channel_id,
                    nonce,
                },
                MockCommitmentValue::ReceivePacket(packet),
            ) => {
                let received_packets = state
                    .received_packets
                    .entry((dst_channel_id.clone(), src_channel_id.clone()))
                    .or_default();

                if received_packets.contains_key(nonce) {
                    return Err("commitment already exist".to_owned());
                }

                received_packets.insert(nonce.clone(), packet.clone());

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
                let sent_packets = state
                    .sent_packets
                    .entry((src_channel_id.clone(), dst_channel_id.clone()))
                    .or_default();

                if sent_packets.contains_key(nonce) {
                    return Err("commitment already exist".to_owned());
                }

                sent_packets.insert(nonce.clone(), packet.clone());

                Ok(())
            }
            _ => Err("invalid commitment".to_owned()),
        }
    }
}
