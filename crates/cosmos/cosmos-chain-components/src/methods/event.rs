use alloc::sync::Arc;

use ibc_relayer::event::extract_packet_and_write_ack_from_tx;
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::events::IbcEventType;
use tendermint::abci::Event as AbciEvent;

pub fn try_extract_write_ack_event(event: &Arc<AbciEvent>) -> Option<WriteAcknowledgement> {
    if let IbcEventType::WriteAck = event.kind.parse().ok()? {
        let (packet, write_ack) = extract_packet_and_write_ack_from_tx(event).ok()?;

        let ack = WriteAcknowledgement {
            packet,
            ack: write_ack,
        };

        Some(ack)
    } else {
        None
    }
}
