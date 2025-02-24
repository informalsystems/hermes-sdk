use cgp::prelude::*;
use hermes_cosmos_chain_components::methods::encode::encode_protobuf;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::{
    ReceivePacketPayloadBuilder, ReceivePacketPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::Height;

use crate::methods::commitment::packet_commitment_bytes;
use crate::methods::encode::sign_data::sign_with_data;
use crate::protobuf::solomachine_v2::PacketCommitmentData;
use crate::traits::solomachine::Solomachine;
use crate::types::client_state::SolomachineClientState;
use crate::types::payloads::packet::SolomachineReceivePacketPayload;
use crate::types::sign_data::SolomachineSignData;

pub struct BuildSolomachineReceivePacketPayload;

#[cgp_provider(ReceivePacketPayloadBuilderComponent)]
impl<Chain, Counterparty> ReceivePacketPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineReceivePacketPayload
where
    Chain: Solomachine
        + HasReceivePacketPayloadType<
            Counterparty,
            ReceivePacketPayload = SolomachineReceivePacketPayload,
        > + HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>
        + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + HasHeightType<Height = Height>
        + HasAsyncErrorType,
{
    async fn build_receive_packet_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        packet: &Packet,
    ) -> Result<SolomachineReceivePacketPayload, Chain::Error> {
        let commitment_bytes = packet_commitment_bytes(packet);

        let commitment_path = format!(
            "commitments/ports/{}/channels/{}/sequences/{}",
            packet.port_id_on_a, packet.chan_id_on_a, packet.seq_on_a
        );

        let packet_commitment_data = PacketCommitmentData {
            path: commitment_path.as_bytes().to_vec(),
            commitment: commitment_bytes,
        };

        let packet_commitment_data_bytes = encode_protobuf(&packet_commitment_data);

        let new_diversifier = chain.current_diversifier();
        let secret_key = chain.secret_key();
        let consensus_timestamp = client_state.consensus_state.timestamp;

        let sign_data = SolomachineSignData {
            sequence: u64::from(packet.seq_on_a),
            timestamp: consensus_timestamp,
            diversifier: new_diversifier,
            data: packet_commitment_data_bytes,
            path: commitment_path.into_bytes(),
        };

        let proof = sign_with_data(secret_key, &sign_data);

        let payload = SolomachineReceivePacketPayload {
            update_height: *height,
            proof_commitment: proof,
        };

        Ok(payload)
    }
}
