use hermes_relayer_components::chain::traits::packet::fields::OutgoingPacketFieldsReader;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use ibc::core::channel::types::packet::Packet;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, PortId, Sequence};
use ibc::primitives::Timestamp;

pub struct CosmosPacketFieldReader;

impl<Chain, Counterparty> OutgoingPacketFieldsReader<Chain, Counterparty>
    for CosmosPacketFieldReader
where
    Chain: HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>
        + HasIbcChainTypes<
            Counterparty,
            Timeout = Timestamp,
            ChannelId = ChannelId,
            PortId = PortId,
            Sequence = Sequence,
        >,
    Counterparty: HasIbcChainTypes<
        Chain,
        Timeout = Timestamp,
        ChannelId = ChannelId,
        PortId = PortId,
        Sequence = Sequence,
    >,
    Chain::Height: From<Height>,
    Counterparty::Height: From<Height>,
{
    fn packet_src_channel_id(packet: &Packet) -> &ChannelId {
        &packet.chan_id_on_a
    }

    fn packet_dst_channel_id(packet: &Packet) -> &ChannelId {
        &packet.chan_id_on_b
    }

    fn packet_src_port(packet: &Packet) -> &PortId {
        &packet.port_id_on_a
    }

    fn packet_dst_port(packet: &Packet) -> &PortId {
        &packet.port_id_on_b
    }

    fn packet_sequence(packet: &Packet) -> &Sequence {
        &packet.seq_on_a
    }

    fn packet_timeout_height(packet: &Packet) -> Option<Counterparty::Height> {
        match &packet.timeout_height_on_b {
            TimeoutHeight::Never => None,
            TimeoutHeight::At(h) => Some((*h).into()),
        }
    }

    fn packet_timeout_timestamp(packet: &Packet) -> Option<Timestamp> {
        match &packet.timeout_timestamp_on_b {
            TimeoutTimestamp::Never => None,
            TimeoutTimestamp::At(timestamp) => Some(*timestamp),
        }
    }
}
