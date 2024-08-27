use std::ops::Add;
use std::time::Duration;

use ibc::apps::transfer::types::msgs::transfer::MsgTransfer;
use ibc::apps::transfer::types::packet::PacketData;
use ibc::apps::transfer::types::BaseCoin;
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, PortId, Sequence};
use ibc::primitives::Timestamp;
use primitive_types::U256;

use super::dummy::dummy_signer;

pub fn build_msg_transfer() -> MsgTransfer {
    MsgTransfer {
        port_id_on_a: PortId::transfer(),
        chan_id_on_a: ChannelId::zero(),
        packet_data: PacketData {
            token: BaseCoin {
                denom: "uatom".parse().unwrap(),
                amount: U256::from(10).into(),
            }
            .into(),
            sender: dummy_signer(),
            receiver: dummy_signer(),
            memo: "".to_string().into(),
        },
        timeout_timestamp_on_b: Timestamp::now()
            .add(Duration::from_secs(10))
            .unwrap()
            .into(),
        timeout_height_on_b: Height::new(0, 1000).unwrap().into(),
    }
}

pub fn build_transfer_packet(sequence: u64) -> Packet {
    let msg = build_msg_transfer();

    let data = serde_json::to_vec(&msg.packet_data)
        .expect("PacketData's infallible Serialize impl failed");

    Packet {
        seq_on_a: Sequence::from(sequence),
        port_id_on_a: msg.port_id_on_a.clone(),
        chan_id_on_a: msg.chan_id_on_a.clone(),
        port_id_on_b: PortId::transfer(),
        chan_id_on_b: ChannelId::zero(),
        data,
        timeout_height_on_b: msg.timeout_height_on_b,
        timeout_timestamp_on_b: msg.timeout_timestamp_on_b,
    }
}
