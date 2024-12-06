use ibc::core::channel::types::channel::Order;
use ibc::core::channel::types::events::SendPacket;
use ibc::core::channel::types::packet::Packet;
use ibc::core::host::types::identifiers::ConnectionId;

pub struct SendPacketEvent {
    pub packet: Packet,
    pub channel_ordering: Order,
    pub connection_id: ConnectionId,
}

impl From<SendPacket> for SendPacketEvent {
    fn from(send_packet: SendPacket) -> Self {
        let packet = Packet {
            seq_on_a: *send_packet.seq_on_a(),
            port_id_on_a: send_packet.port_id_on_a().clone(),
            chan_id_on_a: send_packet.chan_id_on_a().clone(),
            port_id_on_b: send_packet.port_id_on_b().clone(),
            chan_id_on_b: send_packet.chan_id_on_b().clone(),
            data: send_packet.packet_data().to_vec(),
            timeout_height_on_b: *send_packet.timeout_height_on_b(),
            timeout_timestamp_on_b: *send_packet.timeout_timestamp_on_b(),
        };

        Self {
            packet,
            channel_ordering: *send_packet.channel_ordering(),
            connection_id: send_packet.conn_id_on_a().clone(),
        }
    }
}
