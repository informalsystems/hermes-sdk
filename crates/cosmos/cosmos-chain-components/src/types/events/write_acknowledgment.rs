use ibc::core::channel::types::events::WriteAcknowledgement;
use ibc::core::channel::types::packet::Packet;
use ibc::core::host::types::identifiers::ConnectionId;

#[derive(Clone)]
pub struct WriteAckEvent {
    pub packet: Packet,
    pub acknowledgment: Vec<u8>,
    pub connection_id: ConnectionId,
}

impl From<WriteAcknowledgement> for WriteAckEvent {
    fn from(write_acknowledgment: WriteAcknowledgement) -> Self {
        let packet = Packet {
            seq_on_a: *write_acknowledgment.seq_on_a(),
            port_id_on_a: write_acknowledgment.port_id_on_a().clone(),
            chan_id_on_a: write_acknowledgment.chan_id_on_a().clone(),
            port_id_on_b: write_acknowledgment.port_id_on_b().clone(),
            chan_id_on_b: write_acknowledgment.chan_id_on_b().clone(),
            data: write_acknowledgment.packet_data().to_vec(),
            timeout_height_on_b: *write_acknowledgment.timeout_height_on_b(),
            timeout_timestamp_on_b: *write_acknowledgment.timeout_timestamp_on_b(),
        };

        let acknowledgment = write_acknowledgment.acknowledgement().as_bytes().to_vec();

        Self {
            packet,
            acknowledgment,
            connection_id: write_acknowledgment.conn_id_on_b().clone(),
        }
    }
}
