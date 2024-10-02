use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::Async;
use cgp::prelude::CanRaiseError;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::{HasDefaultEncoding, HasEncoding};

use crate::traits::handlers::incoming::packet_entry::IncomingPacketEntryHandler;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::payload::ack::HasPayloadAckType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

pub struct ConvertAndHandlePacketEntry<InApp, InHandler>(pub PhantomData<(InApp, InHandler)>);

impl<
        Chain,
        Counterparty,
        App,
        InApp,
        InHandler,
        AnyPacketData,
        PacketData,
        AnyPacketAck,
        PacketAck,
    > IncomingPacketEntryHandler<Chain, Counterparty, App>
    for ConvertAndHandlePacketEntry<InApp, InHandler>
where
    Chain: HasPayloadAckType<Counterparty, App, PayloadAck = AnyPacketAck>
        + HasPayloadAckType<Counterparty, InApp, PayloadAck = PacketAck>
        + HasEncoding<App>
        + CanRaiseError<ErrorOf<Chain::Encoding>>
        + CanRaiseError<ErrorOf<Counterparty::Encoding>>,
    Counterparty: HasPacketHeaderType<Chain>
        + HasPayloadHeaderType<Chain>
        + HasPacketDataType<Chain, App, PacketData = AnyPacketData>
        + HasPacketDataType<Chain, InApp, PacketData = PacketData>
        + HasDefaultEncoding<App>,
    InHandler: IncomingPacketEntryHandler<Chain, Counterparty, InApp>,
    Chain::Encoding: CanConvert<PacketAck, AnyPacketAck>,
    Counterparty::Encoding: CanConvert<AnyPacketData, PacketData>,
    PacketData: Async,
    AnyPacketData: Async,
{
    async fn handle_incoming_packet_entry(
        chain: &Chain,
        packet_header: &Counterparty::PacketHeader,
        entry_header: &Counterparty::PayloadHeader,
        raw_packet_data: &AnyPacketData,
    ) -> Result<AnyPacketAck, Chain::Error> {
        let packet_data = Counterparty::default_encoding()
            .convert(raw_packet_data)
            .map_err(Chain::raise_error)?;

        let ack = InHandler::handle_incoming_packet_entry(
            chain,
            packet_header,
            entry_header,
            &packet_data,
        )
        .await?;

        let raw_ack = chain.encoding().convert(&ack).map_err(Chain::raise_error)?;

        Ok(raw_ack)
    }
}
