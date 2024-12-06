use ibc::core::channel::types::packet::Packet;
use sha2::{Digest, Sha256};

pub fn packet_commitment_bytes(packet: &Packet) -> Vec<u8> {
    let mut buf = Vec::new();

    let timeout_timestamp = packet.timeout_timestamp_on_b.nanoseconds();
    let timeout_revision_number = packet.timeout_height_on_b.commitment_revision_number();
    let timeout_revision_height = packet.timeout_height_on_b.commitment_revision_height();

    buf.extend(timeout_timestamp.to_be_bytes());
    buf.extend(timeout_revision_number.to_be_bytes());
    buf.extend(timeout_revision_height.to_be_bytes());
    buf.extend(Sha256::digest(&packet.data));

    Sha256::digest(&buf).to_vec()
}
