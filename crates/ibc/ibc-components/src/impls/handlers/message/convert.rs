use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::Async;
use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasEncoding;

use crate::traits::handlers::message::IbcMessageHandler;
use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::message_header::HasIbcMessageHeaderType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

pub struct ConvertAndHandleIbcMessage<InApp, InHandler>(pub PhantomData<(InApp, InHandler)>);

impl<
        Chain,
        Counterparty,
        App,
        InApp,
        InHandler,
        AnyMessage,
        Message,
        AnyPacketData,
        PacketData,
    > IbcMessageHandler<Chain, Counterparty, App> for ConvertAndHandleIbcMessage<InApp, InHandler>
where
    Chain: HasErrorType
        + HasIbcTransactionHeaderType<Counterparty>
        + HasIbcMessageHeaderType<Counterparty>
        + HasIbcMessageType<Counterparty, App, IbcMessage = AnyMessage>
        + HasIbcMessageType<Counterparty, InApp, IbcMessage = Message>
        + HasPacketDataType<Counterparty, App, PacketData = AnyPacketData>
        + HasPacketDataType<Counterparty, InApp, PacketData = PacketData>
        + HasPacketEntryHeaderType<Counterparty>
        + HasEncoding<App>
        + CanRaiseError<ErrorOf<Chain::Encoding>>,
    InHandler: IbcMessageHandler<Chain, Counterparty, InApp>,
    Chain::Encoding: CanConvert<AnyMessage, Message> + CanConvert<PacketData, AnyPacketData>,
    Message: Async,
    AnyMessage: Async,
    AnyPacketData: Async,
{
    async fn handle_ibc_message(
        chain: &Chain,
        transaction_header: &Chain::IbcTransactionHeader,
        message_header: &Chain::IbcMessageHeader,
        any_message: &AnyMessage,
    ) -> Result<(Chain::PacketEntryHeader, AnyPacketData), Chain::Error> {
        let encoding = chain.encoding();

        let message = encoding.convert(any_message).map_err(Chain::raise_error)?;

        let (entry_header, packet_data) =
            InHandler::handle_ibc_message(chain, transaction_header, message_header, &message)
                .await?;

        let any_packet_data = encoding.convert(&packet_data).map_err(Chain::raise_error)?;

        Ok((entry_header, any_packet_data))
    }
}
