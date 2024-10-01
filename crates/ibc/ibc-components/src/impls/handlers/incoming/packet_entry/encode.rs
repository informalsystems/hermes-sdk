use core::marker::PhantomData;

use cgp::prelude::HasErrorType;

use crate::traits::handlers::incoming::packet_entry::IncomingPacketEntryHandler;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;

pub struct EncodeAndHandlePacketEntry<InApp>(pub PhantomData<InApp>);

impl<Chain, Counterparty, App, InApp> IncomingPacketEntryHandler<Chain, Counterparty, App>
    for EncodeAndHandlePacketEntry<InApp>
where
    Chain: HasErrorType + HasPacketAckType<Counterparty, App>,
    Counterparty: HasPacketHeaderType<Chain>
        + HasPacketEntryHeaderType<Chain>
        + HasPacketDataType<Chain, App>,
{
    async fn handle_incoming_packet_entry(
        _chain: &Chain,
        _packet_header: &Counterparty::PacketHeader,
        _entry_header: &Counterparty::PacketEntryHeader,
        _entry_data: &Counterparty::PacketData,
    ) -> Result<Chain::PacketAck, Chain::Error> {
        todo!()
    }
}
