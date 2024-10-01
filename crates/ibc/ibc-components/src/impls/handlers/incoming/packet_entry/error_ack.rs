use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::fields::packet::ack::error::ErrorAsAckWrapper;
use crate::traits::handlers::incoming::packet_entry::IncomingPacketEntryHandler;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::entry_ack::HasPacketEntryAckType;
use crate::traits::types::packet::header::HasPacketHeaderType;

pub struct WrapHandlerErrorAsAck<ErrorHandler, InHandler>(
    pub PhantomData<(ErrorHandler, InHandler)>,
);

impl<Chain, Counterparty, App, ErrorHandler, InHandler>
    IncomingPacketEntryHandler<Chain, Counterparty, App>
    for WrapHandlerErrorAsAck<ErrorHandler, InHandler>
where
    Chain: HasErrorType + HasPacketEntryAckType<Counterparty, App>,
    Counterparty: HasPacketHeaderType<Chain>
        + HasPacketEntryHeaderType<Chain>
        + HasPacketDataType<Chain, App>,
    InHandler: IncomingPacketEntryHandler<Chain, Counterparty, App>,
    ErrorHandler: ErrorAsAckWrapper<Chain, Counterparty, App>,
{
    async fn handle_incoming_packet_entry(
        chain: &Chain,
        packet_header: &Counterparty::PacketHeader,
        entry_header: &Counterparty::PacketEntryHeader,
        entry_data: &Counterparty::PacketData,
    ) -> Result<Chain::PacketAck, Chain::Error> {
        let res =
            InHandler::handle_incoming_packet_entry(chain, packet_header, entry_header, entry_data)
                .await;

        match res {
            Ok(ack) => Ok(ack),
            Err(e) => ErrorHandler::try_wrap_error_as_ack(e),
        }
    }
}
