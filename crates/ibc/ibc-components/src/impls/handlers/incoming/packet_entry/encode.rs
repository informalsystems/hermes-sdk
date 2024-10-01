use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::Async;
use cgp::prelude::CanRaiseError;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::{HasDefaultEncoding, HasEncoding};

use crate::traits::handlers::incoming::packet_entry::{
    CanHandleIncomingPacketEntry, IncomingPacketEntryHandler,
};
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;

pub struct EncodeAndHandlePacketEntry<InApp>(pub PhantomData<InApp>);

impl<Chain, Counterparty, App, InApp, RawPacketData, PacketData, RawPacketAck, PacketAck>
    IncomingPacketEntryHandler<Chain, Counterparty, App> for EncodeAndHandlePacketEntry<InApp>
where
    Chain: HasPacketAckType<Counterparty, App, PacketAck = RawPacketAck>
        + HasPacketAckType<Counterparty, InApp, PacketAck = PacketAck>
        + CanHandleIncomingPacketEntry<Counterparty, InApp>
        + HasEncoding<App>
        + CanRaiseError<ErrorOf<Chain::Encoding>>
        + CanRaiseError<ErrorOf<Counterparty::Encoding>>,
    Counterparty: HasPacketHeaderType<Chain>
        + HasPacketEntryHeaderType<Chain>
        + HasPacketDataType<Chain, App, PacketData = RawPacketData>
        + HasPacketDataType<Chain, InApp, PacketData = PacketData>
        + HasDefaultEncoding<App>,
    Chain::Encoding: CanConvert<PacketAck, RawPacketAck>,
    Counterparty::Encoding: CanConvert<RawPacketData, PacketData>,
    PacketData: Async,
    RawPacketData: Async,
{
    async fn handle_incoming_packet_entry(
        chain: &Chain,
        packet_header: &Counterparty::PacketHeader,
        entry_header: &Counterparty::PacketEntryHeader,
        raw_packet_data: &RawPacketData,
    ) -> Result<RawPacketAck, Chain::Error> {
        let packet_data = Counterparty::default_encoding()
            .convert(raw_packet_data)
            .map_err(Chain::raise_error)?;

        let ack = chain
            .handle_incoming_packet_entry(packet_header, entry_header, &packet_data)
            .await?;

        let raw_ack = chain.encoding().convert(&ack).map_err(Chain::raise_error)?;

        Ok(raw_ack)
    }
}
