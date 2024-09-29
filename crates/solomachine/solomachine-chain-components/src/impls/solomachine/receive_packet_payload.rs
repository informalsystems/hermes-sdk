use cgp::core::error::HasErrorType;
use hermes_cosmos_chain_components::methods::encode::encode_protobuf;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::path::CommitmentsPath;
use ibc_relayer_types::Height;

use crate::methods::commitment::packet_commitment_bytes;
use crate::methods::encode::sign_data::sign_with_data;
use crate::protobuf::solomachine_v2::PacketCommitmentData;
use crate::traits::solomachine::Solomachine;
use crate::types::client_state::SolomachineClientState;
use crate::types::payloads::packet::SolomachineReceivePacketPayload;
use crate::types::sign_data::SolomachineSignData;

pub struct BuildSolomachineReceivePacketPayload;

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
        + HasErrorType,
{
    async fn build_receive_packet_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        packet: &Packet,
    ) -> Result<SolomachineReceivePacketPayload, Chain::Error> {
        let commitment_bytes = packet_commitment_bytes(packet);

        let commitment_path = CommitmentsPath {
            port_id: packet.source_port.clone(),
            channel_id: packet.source_channel.clone(),
            sequence: packet.sequence,
        };

        let commitment_path = commitment_path.to_string();

        let packet_commitment_data = PacketCommitmentData {
            path: commitment_path.as_bytes().to_vec(),
            commitment: commitment_bytes,
        };

        let packet_commitment_data_bytes = encode_protobuf(&packet_commitment_data);

        let new_diversifier = chain.current_diversifier();
        let secret_key = chain.secret_key();
        let consensus_timestamp = client_state.consensus_state.timestamp;

        let sign_data = SolomachineSignData {
            sequence: u64::from(packet.sequence),
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
