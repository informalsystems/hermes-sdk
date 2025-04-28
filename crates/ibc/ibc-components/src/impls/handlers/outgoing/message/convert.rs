use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::prelude::*;
use hermes_encoding_components::traits::{CanConvert, HasEncoding};

use crate::traits::handlers::outgoing::message::{IbcMessageHandler, IbcMessageHandlerComponent};
use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::message_header::HasIbcMessageHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

pub struct ConvertAndHandleIbcMessage<InApp, InHandler>(pub PhantomData<(InApp, InHandler)>);

#[cgp_provider(IbcMessageHandlerComponent)]
#[async_trait]
impl<
        Chain,
        Counterparty,
        App,
        InApp,
        InHandler,
        AnyMessage,
        Message,
        AnyPayloadData,
        PayloadData,
    > IbcMessageHandler<Chain, Counterparty, App> for ConvertAndHandleIbcMessage<InApp, InHandler>
where
    Chain: HasAsyncErrorType
        + HasPacketHeaderType<Counterparty>
        + HasIbcMessageHeaderType<Counterparty>
        + HasIbcMessageType<Counterparty, App, IbcMessage = AnyMessage>
        + HasIbcMessageType<Counterparty, InApp, IbcMessage = Message>
        + HasPayloadDataType<Counterparty, App, PayloadData = AnyPayloadData>
        + HasPayloadDataType<Counterparty, InApp, PayloadData = PayloadData>
        + HasPayloadHeaderType<Counterparty>
        + HasEncoding<App>
        + CanRaiseAsyncError<ErrorOf<Chain::Encoding>>,
    InHandler: IbcMessageHandler<Chain, Counterparty, InApp>,
    Chain::Encoding:
        CanConvert<AnyMessage, Message> + CanConvert<PayloadData, AnyPayloadData> + Clone,
    Message: Async,
    AnyMessage: Async,
{
    async fn handle_ibc_message(
        chain: &mut Chain,
        packet_header: &Chain::PacketHeader,
        message_header: &Chain::IbcMessageHeader,
        any_message: &AnyMessage,
    ) -> Result<(Chain::PayloadHeader, AnyPayloadData), Chain::Error> {
        let encoding = chain.encoding().clone();

        let message = encoding.convert(any_message).map_err(Chain::raise_error)?;

        let (payload_header, any_payload_data) =
            InHandler::handle_ibc_message(chain, packet_header, message_header, &message).await?;

        let payload_data = encoding
            .convert(&any_payload_data)
            .map_err(Chain::raise_error)?;

        Ok((payload_header, payload_data))
    }
}
